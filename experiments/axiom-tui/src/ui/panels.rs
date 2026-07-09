use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

use crate::{
    app::App,
    models::{owned_line, Panel, ViewId},
    theme::{
        C_AMBER, C_BG2, C_BG3, C_BLUE, C_BORDER, C_CYAN, C_DIM, C_GREEN, C_MUTED, C_PURPLE, C_RED,
        C_TEXT,
    },
};

use super::{
    layout::{panel_shell, render_empty_state, render_hint, table_state_for},
    tables::{audit_row, bias_row, compliance_row, loss_row, model_row, risk_row, submission_row},
};

pub fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    match app.active_panel {
        Panel::Dashboard => draw_dashboard(f, app, area),
        Panel::RiskRegistry => draw_risk_registry(f, app, area),
        Panel::Submissions => draw_submissions(f, app, area),
        Panel::PolicyConfig => draw_static_panel(
            f,
            area,
            "POLICY CONFIG",
            &[
                "Configure coverage clauses, exclusions, AI-specific endorsements",
                "and manuscript policy terms.",
                "",
                "Supported: occurrence / claims-made, cyber AI endorsement,",
                "algorithmic decision exclusions, model version pinning clauses.",
            ],
            app.hint_text_for(app.active_view()),
        ),
        Panel::QuoteEngine => draw_quote_engine(f, app, area),
        Panel::LossModels => draw_loss_models(f, app, area),
        Panel::ExposureAnalysis => draw_static_panel(
            f,
            area,
            "EXPOSURE ANALYSIS - BY AI RISK SEGMENT",
            &[
                "SEGMENT           TIV         EML         PML (1-in-100)  LIMIT UTIL",
                "AI Liability      $28.4M      $12.1M      $8.6M           72%",
                "Cyber (AI)        $15.2M      $9.8M       $6.2M           55%",
                "D&O Governance    $42.0M      $16.0M      $10.4M          38%",
                "E&O Algorithmic   $19.5M      $11.9M      $7.8M           61%",
                "Product Liability $11.0M      $3.2M       $2.1M           29%",
                "",
                "PML basis: RMS AI Cat Model v2.1  |  Corr factor: 0.38",
            ],
            app.hint_text_for(app.active_view()),
        ),
        Panel::Reserves => draw_reserves(f, app, area),
        Panel::Retrospective => draw_static_panel(
            f,
            area,
            "RETROSPECTIVE ANALYSIS - EXPERIENCE RATING",
            &[
                "BUHLMANN CREDIBILITY RATING",
                "",
                "Insured: Apex Autonomy Ltd  (AI Liability line)",
                "Years:   5  (2020-2024)",
                "",
                "Observed pure premium   $1.42 per $1,000 TIV",
                "Manual pure premium     $1.10 per $1,000 TIV",
                "Credibility weight (Z)  0.71",
                "Credibility-weighted PP $1.32",
                "",
                "Combined retro mod      1.05  -> +5% loading applied",
            ],
            app.hint_text_for(app.active_view()),
        ),
        Panel::ModelRegistry => draw_model_registry(f, app, area),
        Panel::BiasMonitor => draw_bias_monitor(f, app, area),
        Panel::DriftDetection => draw_drift(f, app, area),
        Panel::Explainability => draw_explainability(f, app, area),
        Panel::AuditTrail => draw_audit(f, app, area),
        Panel::Compliance => draw_compliance(f, app, area),
        Panel::Reporting => draw_reporting(f, app, area),
        Panel::Config => draw_static_panel(
            f,
            area,
            "SYSTEM CONFIG",
            &[
                "Database connections, model endpoint registry,",
                "alert thresholds, keybindings, and theme.",
                "",
                "DB:     postgres://axiom@localhost:5432/actuarial",
                "Models: http://model-svc:8080",
                "Alerts: pagerduty + slack #ai-underwriting",
            ],
            app.hint_text_for(app.active_view()),
        ),
        Panel::LiveLogs => draw_live_logs(f, app, area),
    }
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "PORTFOLIO AT A GLANCE");
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(shell.content);

    let card_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(rows[0]);

    let metrics = [
        ("GROSS WRITTEN PREMIUM", "$4.2M", "▲ 8.3% QoQ", C_GREEN),
        ("COMBINED RATIO", "97.4%", "target < 98%", C_AMBER),
        ("OPEN SUBMISSIONS", "23", "3 flagged by AI", C_BLUE),
    ];
    for (index, (label, value, sub, color)) in metrics.iter().enumerate() {
        draw_metric_card(f, card_chunks[index], label, value, sub, *color);
    }

    let gauge_area = rows[2];
    let gauges = [
        ("AI Liability", 72u16, C_BLUE),
        ("Cyber (AI-induced)", 55, C_PURPLE),
        ("D&O / Governance", 38, C_CYAN),
        ("E&O (Algorithmic)", 61, C_AMBER),
        ("Product Liability", 29, C_GREEN),
        ("IBNR Reserve %", 44, C_RED),
    ];

    f.render_widget(
        Paragraph::new("  RISK EXPOSURE BY LINE  (% of limit deployed)")
            .style(Style::default().fg(C_CYAN)),
        Rect {
            x: gauge_area.x,
            y: gauge_area.y,
            width: gauge_area.width,
            height: 1,
        },
    );

    for (index, (label, pct, color)) in gauges.iter().enumerate() {
        let y = gauge_area.y + 1 + index as u16;
        if y >= gauge_area.y + gauge_area.height {
            break;
        }
        let row = Rect {
            x: gauge_area.x,
            y,
            width: gauge_area.width,
            height: 1,
        };
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(22),
                Constraint::Min(0),
                Constraint::Length(5),
            ])
            .split(row);
        f.render_widget(
            Paragraph::new(*label).style(Style::default().fg(C_MUTED)),
            cols[0],
        );
        f.render_widget(
            Gauge::default()
                .gauge_style(Style::default().fg(*color).bg(C_BG3))
                .percent(*pct)
                .label(""),
            cols[1],
        );
        f.render_widget(
            Paragraph::new(format!("{pct}%"))
                .style(Style::default().fg(*color))
                .alignment(Alignment::Right),
            cols[2],
        );
    }

    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_metric_card(
    f: &mut Frame,
    area: Rect,
    label: &str,
    val: &str,
    sub: &str,
    color: ratatui::style::Color,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(C_BORDER))
        .style(Style::default().bg(C_BG2));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);
    f.render_widget(
        Paragraph::new(label)
            .style(Style::default().fg(C_MUTED))
            .alignment(Alignment::Center),
        rows[0],
    );
    f.render_widget(
        Paragraph::new(val)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center),
        rows[1],
    );
    f.render_widget(
        Paragraph::new(sub)
            .style(Style::default().fg(C_DIM))
            .alignment(Alignment::Center),
        rows[2],
    );
}

fn draw_risk_registry(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "AI RISK TAXONOMY - REGISTRY");
    let view = ViewId::RiskRegistry;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| risk_row(&app.risks[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "ID",
        "CATEGORY",
        "DESCRIPTION",
        "FREQ",
        "SEVERITY",
        "MITIGATION",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD))
    .height(1);
    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Min(24),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Min(14),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3).fg(C_TEXT))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, shell.content, &mut state);
    render_empty_state(f, app.visible_row_count_for(view), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(view));
}

fn draw_submissions(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "SUBMISSIONS");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(shell.content);
    let tabs = Tabs::new(vec!["Queue", "New Submission", "AI Triage"])
        .select(app.tab_index)
        .style(Style::default().fg(C_MUTED))
        .highlight_style(Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD))
        .divider("│");
    f.render_widget(tabs, layout[0]);

    match app.active_view() {
        ViewId::SubmissionsQueue => draw_submission_queue(f, app, layout[1]),
        ViewId::SubmissionsForm => draw_new_submission_form(f, layout[1]),
        ViewId::SubmissionsTriage => draw_ai_triage(f, app, layout[1]),
        _ => {}
    }
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_submission_queue(f: &mut Frame, app: &App, area: Rect) {
    let view = ViewId::SubmissionsQueue;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| submission_row(&app.submissions[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "REF", "INSURED", "LINE", "LIMIT", "AI SCORE", "FLAG", "STATUS",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Min(18),
            Constraint::Length(7),
            Constraint::Length(6),
            Constraint::Length(11),
            Constraint::Min(18),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, area, &mut state);
    render_empty_state(f, app.visible_row_count_for(view), area);
}

fn draw_new_submission_form(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![Span::styled(
            "  NEW SUBMISSION - AI ASSURANCE LINE",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        form_line("Insured Name", "________________"),
        form_line("Line of Business", "[AI Liability ▾]"),
        form_line("Policy Limit", "________________"),
        form_line("Retention", "________________"),
        form_line("Policy Period", "[12 months ▾]"),
        form_line("AI Model Count", "________________"),
        form_line("Deployment Env", "[Production ▾]"),
        form_line("GDPR / CCPA Scope", "[Yes ▾]"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  [ Submit for AI Triage ]",
                Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("[ Clear ]", Style::default().fg(C_MUTED)),
        ]),
    ];
    f.render_widget(Paragraph::new(lines), area);
}

fn form_line(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  {label:<18}: "), Style::default().fg(C_MUTED)),
        Span::styled(value.to_string(), Style::default().fg(C_BLUE)),
    ])
}

fn draw_ai_triage(f: &mut Frame, app: &App, area: Rect) {
    let triage = &app.triage;
    let composite_color = if triage.composite_score >= 0.85 {
        C_RED
    } else if triage.composite_score >= 0.70 {
        C_AMBER
    } else {
        C_GREEN
    };
    let lines = vec![
        Line::from(vec![Span::styled(
            "  AI TRIAGE ENGINE - AUTOMATED RISK ASSESSMENT",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        owned_line("  Scenario", &triage.scenario, C_BLUE),
        owned_line("  Iterations", &triage.iterations.to_string(), C_MUTED),
        owned_line("  Model", "RiskScore-v3 (M-01)", C_TEXT),
        owned_line("  Run mode", "Scenario workflow", C_MUTED),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  SCORING DIMENSIONS",
            Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD),
        )]),
        triage_metric_line("  Model Risk Score", triage.model_score, C_AMBER),
        triage_metric_line("  Data Provenance", triage.data_score, C_AMBER),
        triage_metric_line("  Fairness Score", triage.fairness_score, C_GREEN),
        triage_metric_line("  Explainability", triage.explainability_score, C_GREEN),
        triage_metric_line("  Operational Risk", triage.operational_score, C_AMBER),
        Line::from(""),
        Line::from(vec![
            Span::styled("  COMPOSITE RISK  ", Style::default().fg(C_MUTED)),
            Span::styled(
                format!("{} {:.2}", triage.composite_label, triage.composite_score),
                Style::default()
                    .fg(composite_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  -> {}", triage.recommendation),
                Style::default().fg(C_MUTED),
            ),
        ]),
    ];
    f.render_widget(Paragraph::new(lines), area);
}

fn triage_metric_line(label: &str, score: f64, color: ratatui::style::Color) -> Line<'static> {
    let pct = (score * 100.0).round() as u16;
    Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(C_MUTED)),
        Span::styled(
            format!("  {:.2}  {}", score, bar(pct)),
            Style::default().fg(color),
        ),
    ])
}

fn draw_quote_engine(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "QUOTE ENGINE - AI ASSURANCE LINE");
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(shell.content);

    let left = vec![
        Line::from(vec![Span::styled(
            "  SUBMISSION DETAILS",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        owned_line("  Insured", "Apex Autonomy Ltd", C_TEXT),
        owned_line("  Line", "AI Liability", C_MUTED),
        owned_line("  Policy Limit", "$5,000,000", C_TEXT),
        owned_line("  Retention", "$250,000", C_MUTED),
        owned_line("  Period", "12 months", C_MUTED),
        owned_line("  AI Models", "14 (3 high-risk)", C_TEXT),
        owned_line("  Deployment", "Production / EU", C_MUTED),
        owned_line("  GDPR scope", "Yes", C_AMBER),
        Line::from(""),
        owned_line("  Quote owner", &app.quote.owner, C_BLUE),
        owned_line("  Urgency", &app.quote.urgency, C_AMBER),
        owned_line("  Routing", &app.quote.routing_status, C_CYAN),
        owned_line(
            "  Referral target",
            app.quote.referral_target.as_deref().unwrap_or("None"),
            C_PURPLE,
        ),
        owned_line(
            "  Referral reason",
            app.quote.referral_reason.as_deref().unwrap_or("-"),
            C_DIM,
        ),
    ];
    f.render_widget(Paragraph::new(left), cols[0]);

    let right = vec![
        Line::from(vec![Span::styled(
            "  RATE BUILD-UP",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        rate_line("  Base rate", "1.80%", C_BLUE),
        rate_line("  Model risk load", "0.43%", C_AMBER),
        rate_line("  Bias surcharge", "0.15%", C_RED),
        rate_line("  Cat load", "0.22%", C_RED),
        rate_line("  Explainability credit", "-0.10%", C_GREEN),
        rate_line("  GDPR load", "0.08%", C_AMBER),
        rate_line("  ULAE load", "0.08%", C_MUTED),
        Line::from(vec![Span::styled(
            "  ---------------------------------------------",
            Style::default().fg(C_BORDER),
        )]),
        owned_line("  Final rate", "2.66%", C_GREEN),
        Line::from(""),
        owned_line("  Indicated premium", "$133,000", C_GREEN),
        Line::from(""),
        Line::from(vec![
            Span::styled("[ a Assign ]", Style::default().fg(C_BLUE)),
            Span::raw("  "),
            Span::styled("[ r Refer ]", Style::default().fg(C_AMBER)),
        ]),
    ];
    f.render_widget(Paragraph::new(right), cols[1]);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn rate_line(label: &str, value: &str, color: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(C_MUTED)),
        Span::styled(
            format!("  {value}  {}", "█".repeat(8)),
            Style::default().fg(color),
        ),
    ])
}

fn draw_loss_models(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "ACTUARIAL LOSS MODELS - AI LIABILITY LINE");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(shell.content);
    let tabs = Tabs::new(vec!["Development Triangles", "Trend Analysis"])
        .select(app.tab_index)
        .style(Style::default().fg(C_MUTED))
        .highlight_style(Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD))
        .divider("│");
    f.render_widget(tabs, layout[0]);

    match app.active_view() {
        ViewId::LossModelsTriangles => draw_loss_triangles(f, app, layout[1]),
        ViewId::LossModelsTrend => draw_loss_trend(f, app, layout[1]),
        _ => {}
    }
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_loss_triangles(f: &mut Frame, app: &App, area: Rect) {
    let summary = &app.loss_summary;
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    let metrics = vec![
        Line::from(vec![
            Span::styled("  Scenario: ", Style::default().fg(C_MUTED)),
            Span::styled(
                &summary.scenario,
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            ),
            Span::styled("    ELR: ", Style::default().fg(C_MUTED)),
            Span::styled(
                format!("{:.1}%", summary.expected_loss_ratio_pct),
                Style::default().fg(C_AMBER).add_modifier(Modifier::BOLD),
            ),
            Span::styled("    Ultimate: ", Style::default().fg(C_MUTED)),
            Span::styled(
                format_money(summary.ultimate_loss),
                Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Tail factor: ", Style::default().fg(C_MUTED)),
            Span::styled(
                format!("{:.2}", summary.tail_factor),
                Style::default().fg(C_PURPLE).add_modifier(Modifier::BOLD),
            ),
            Span::styled("    Iterations: ", Style::default().fg(C_MUTED)),
            Span::styled(summary.iterations.to_string(), Style::default().fg(C_DIM)),
        ]),
    ];
    f.render_widget(Paragraph::new(metrics), layout[0]);

    let view = ViewId::LossModelsTriangles;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| loss_row(&app.loss_rows[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "AY", "12mo", "24mo", "36mo", "48mo", "60mo", "ULTIMATE", "METHOD",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(10),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, layout[1], &mut state);
    render_empty_state(f, app.visible_row_count_for(view), layout[1]);
}

fn draw_loss_trend(f: &mut Frame, app: &App, area: Rect) {
    let summary = &app.loss_summary;
    let lines = vec![
        Line::from(vec![Span::styled(
            "  LOSS TREND ANALYSIS - AI LIABILITY",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        owned_line("  Scenario", &summary.scenario, C_BLUE),
        owned_line("  Iterations", &summary.iterations.to_string(), C_MUTED),
        owned_line(
            "  Severity trend",
            &format!("{:+.1}% p.a.", summary.severity_trend_pct),
            C_RED,
        ),
        owned_line(
            "  Frequency trend",
            &format!("{:+.1}% p.a.", summary.frequency_trend_pct),
            C_AMBER,
        ),
        owned_line(
            "  Pure premium trend",
            &format!("{:+.1}% p.a.", summary.pure_premium_trend_pct),
            C_RED,
        ),
        Line::from(""),
        owned_line("  Note", &summary.note, C_DIM),
    ];
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn draw_reserves(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "RESERVE ADEQUACY - IBNR & CASE");
    let reserves = &app.reserves;
    let lines = vec![
        Line::from(vec![Span::styled(
            "  RESERVE SUMMARY",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        owned_line("  Scenario", &reserves.scenario, C_BLUE),
        owned_line("  Iterations", &reserves.iterations.to_string(), C_MUTED),
        owned_line(
            "  Case reserves",
            &format_money(reserves.case_reserves),
            C_TEXT,
        ),
        owned_line(
            "  Selected IBNR",
            &format_money(reserves.selected_ibnr),
            C_AMBER,
        ),
        owned_line("  ULAE load", &format_money(reserves.ulae), C_TEXT),
        owned_line(
            "  Total reserves",
            &format_money(reserves.total_reserves),
            C_GREEN,
        ),
        Line::from(""),
        owned_line(
            "  Confidence level",
            &format!("{}%", reserves.confidence_pct),
            C_GREEN,
        ),
        owned_line("  CoV", &format!("{:.2}", reserves.cov), C_AMBER),
        owned_line(
            "  Risk margin (IFRS 17)",
            &format_money(reserves.risk_margin),
            C_TEXT,
        ),
        owned_line(
            "  Reserve status",
            &reserves.status,
            if reserves.status == "WATCH" {
                C_AMBER
            } else {
                C_GREEN
            },
        ),
    ];
    f.render_widget(Paragraph::new(lines), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_model_registry(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "AI MODEL REGISTRY - GOVERNANCE");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(shell.content);
    let tabs = Tabs::new(vec!["Models", "Governance Gates"])
        .select(app.tab_index)
        .style(Style::default().fg(C_MUTED))
        .highlight_style(Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD))
        .divider("│");
    f.render_widget(tabs, layout[0]);

    match app.active_view() {
        ViewId::ModelRegistryModels => draw_registry_models(f, app, layout[1]),
        ViewId::ModelRegistryGates => draw_registry_gates(f, layout[1]),
        _ => {}
    }
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_registry_models(f: &mut Frame, app: &App, area: Rect) {
    let view = ViewId::ModelRegistryModels;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| model_row(&app.models[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "ID", "NAME", "PURPOSE", "VER", "DRIFT", "BIAS", "XPLAN", "STATUS",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Length(14),
            Constraint::Min(14),
            Constraint::Length(6),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, area, &mut state);
    render_empty_state(f, app.visible_row_count_for(view), area);
}

fn draw_registry_gates(f: &mut Frame, area: Rect) {
    let gauges = [
        ("Validation coverage", 88u16, C_GREEN),
        ("Explainability ready", 80, C_CYAN),
        ("Bias cleared", 60, C_AMBER),
        ("Drift within bounds", 75, C_BLUE),
        ("Audit trail complete", 95, C_GREEN),
    ];
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "  MODEL GOVERNANCE GATE COMPLIANCE",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
    ];
    for (label, pct, color) in gauges {
        lines.push(Line::from(vec![
            Span::styled(format!("  {label:<26}"), Style::default().fg(C_MUTED)),
            Span::styled(
                format!("{pct:>3}%  "),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(bar(pct), Style::default().fg(color)),
        ]));
    }
    f.render_widget(Paragraph::new(lines), area);
}

fn draw_bias_monitor(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "BIAS & FAIRNESS MONITOR - LIVE");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(shell.content);

    let metrics = vec![
        Line::from(vec![
            Span::styled("  Disparate Impact Ratio: ", Style::default().fg(C_MUTED)),
            Span::styled(
                "0.81  ",
                Style::default().fg(C_AMBER).add_modifier(Modifier::BOLD),
            ),
            Span::styled("(threshold 0.80)    ", Style::default().fg(C_MUTED)),
            Span::styled("Equalized Odds Gap: ", Style::default().fg(C_MUTED)),
            Span::styled(
                "0.09  ",
                Style::default().fg(C_RED).add_modifier(Modifier::BOLD),
            ),
            Span::styled("▲ BREACHED    ", Style::default().fg(C_RED)),
            Span::styled("Calibration ECE: ", Style::default().fg(C_MUTED)),
            Span::styled(
                "0.023",
                Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];
    f.render_widget(Paragraph::new(metrics), layout[0]);

    let view = ViewId::BiasMonitor;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| bias_row(&app.bias_rows[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "ATTRIBUTE",
        "GROUP",
        "APPR RATE",
        "DI RATIO",
        "FPR",
        "FNR",
        "STATUS",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, layout[1], &mut state);
    render_empty_state(f, app.visible_row_count_for(view), layout[1]);
    render_hint(f, shell.hint, &app.hint_text_for(view));
}

fn draw_drift(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "MODEL DRIFT DETECTION - REAL TIME");
    let psi_anim = 0.28 + (app.tick_count as f64 * 0.001).sin() * 0.01;
    let gauges = [
        ("model_version_age", 82u16, C_RED),
        ("claim_frequency", 55, C_AMBER),
        ("deployment_env", 20, C_GREEN),
        ("input_token_dist", 70, C_AMBER),
        ("industry_sector_mix", 40, C_GREEN),
        ("policy_limit_dist", 33, C_GREEN),
    ];
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "  DRIFT METRICS",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  PSI  (PricingLLM)      : ", Style::default().fg(C_MUTED)),
            Span::styled(
                format!("{psi_anim:.3}  "),
                Style::default().fg(C_RED).add_modifier(Modifier::BOLD),
            ),
            Span::styled("threshold 0.20  ▲ BREACHED", Style::default().fg(C_RED)),
        ]),
        owned_line(
            "  KL Divergence",
            "0.140 (ClaimPredict input dist rising)",
            C_AMBER,
        ),
        owned_line("  Accuracy delta", "-0.8% within tolerance", C_GREEN),
        owned_line("  F1 delta", "-1.4% approaching threshold", C_AMBER),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  FEATURE DRIFT BY VARIABLE",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
    ];
    for (label, pct, color) in gauges {
        lines.push(Line::from(vec![
            Span::styled(format!("  {label:<24}"), Style::default().fg(C_MUTED)),
            Span::styled(bar(pct), Style::default().fg(color)),
            Span::styled(
                format!("  {pct}%"),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Action: PricingLLM scheduled for retraining - next run 2025-04-25 02:00 UTC",
        Style::default().fg(C_DIM),
    )]));
    f.render_widget(Paragraph::new(lines), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_explainability(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(
        f,
        area,
        "MODEL EXPLAINABILITY - SHAP / LIME / COUNTERFACTUAL",
    );
    let text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "  DECISION: UW-2941  Apex Autonomy Ltd  -  RiskScore-v3  ->  HIGH 0.87",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  SHAP WATERFALL (top contributors to risk score)",
            Style::default().fg(C_MUTED),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled("  E[f(x)] = 0.38 (base rate)", Style::default().fg(C_DIM))]),
        Line::from(vec![
            Span::styled("  model_hallucination_rate  +0.18  ", Style::default().fg(C_RED)),
            Span::styled("██████████████████░░░░░░░░░░░░░░░", Style::default().fg(C_RED)),
        ]),
        Line::from(vec![
            Span::styled("  no_human_in_loop         +0.14  ", Style::default().fg(C_RED)),
            Span::styled("██████████████░░░░░░░░░░░░░░░░░░░", Style::default().fg(C_RED)),
        ]),
        Line::from(vec![
            Span::styled("  training_data_unlabeled  +0.09  ", Style::default().fg(C_AMBER)),
            Span::styled("█████████░░░░░░░░░░░░░░░░░░░░░░░░", Style::default().fg(C_AMBER)),
        ]),
        Line::from(vec![
            Span::styled("  deployment_prod_only      +0.06  ", Style::default().fg(C_AMBER)),
            Span::styled("██████░░░░░░░░░░░░░░░░░░░░░░░░░░░", Style::default().fg(C_AMBER)),
        ]),
        Line::from(vec![
            Span::styled("  gdpr_scope_eu             +0.04  ", Style::default().fg(C_AMBER)),
            Span::styled("████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░", Style::default().fg(C_AMBER)),
        ]),
        Line::from(vec![
            Span::styled("  shap_cert_present         -0.02  ", Style::default().fg(C_GREEN)),
            Span::styled("██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░", Style::default().fg(C_GREEN)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  COUNTERFACTUAL: add human review, lower hallucination rate, and label training provenance.",
            Style::default().fg(C_DIM),
        )]),
    ];
    f.render_widget(Paragraph::new(text), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_audit(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "AUDIT TRAIL - CRYPTOGRAPHICALLY SIGNED DECISIONS");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(shell.content);
    let mut header_lines = vec![Line::from(vec![Span::styled(
        "  All entries signed (Ed25519).",
        Style::default().fg(C_DIM),
    )])];
    if let Some(export) = app.last_export_for(ViewId::AuditTrail) {
        header_lines.push(Line::from(vec![Span::styled(
            format!(
                "  Last export: {} for {} at {}",
                export.format, export.audience, export.created_at
            ),
            Style::default().fg(C_CYAN),
        )]));
    } else {
        header_lines.push(Line::from(""));
    }
    f.render_widget(Paragraph::new(header_lines), layout[0]);

    let view = ViewId::AuditTrail;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| audit_row(&app.audit_rows[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "TIMESTAMP",
        "ACTOR",
        "ACTION",
        "MODEL",
        "OUTCOME",
        "HASH",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(19),
            Constraint::Length(14),
            Constraint::Length(10),
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, layout[1], &mut state);
    render_empty_state(f, app.visible_row_count_for(view), layout[1]);
    render_hint(f, shell.hint, &app.hint_text_for(view));
}

fn draw_compliance(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "REGULATORY COMPLIANCE STATUS");
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(shell.content);
    let mut top_lines = vec![Line::from(vec![Span::styled(
        "  Active frameworks: EU AI Act, NAIC, GDPR, SOC 2, SR 11-7",
        Style::default().fg(C_DIM),
    )])];
    if let Some(export) = app.last_export_for(ViewId::Compliance) {
        top_lines.push(Line::from(vec![Span::styled(
            format!(
                "  Last export: {} for {} at {}",
                export.format, export.audience, export.created_at
            ),
            Style::default().fg(C_CYAN),
        )]));
    } else {
        top_lines.push(Line::from(""));
    }
    f.render_widget(Paragraph::new(top_lines), layout[0]);

    let view = ViewId::Compliance;
    let rows = app
        .visible_indices(view)
        .into_iter()
        .map(|idx| compliance_row(&app.compliance_rows[idx]))
        .collect::<Vec<_>>();
    let header = Row::new(vec![
        "FRAMEWORK",
        "SCOPE",
        "LAST AUDIT",
        "FINDINGS",
        "STATUS",
    ])
    .style(Style::default().fg(C_MUTED).add_modifier(Modifier::BOLD));
    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Min(16),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .highlight_style(Style::default().bg(C_BG3))
    .highlight_symbol("▶ ");
    let mut state = table_state_for(app, view);
    f.render_stateful_widget(table, layout[1], &mut state);
    render_empty_state(f, app.visible_row_count_for(view), layout[1]);
    render_hint(f, shell.hint, &app.hint_text_for(view));
}

fn draw_reporting(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "REPORTING");
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "  AVAILABLE OUTPUTS",
            Style::default().fg(C_CYAN),
        )]),
        Line::from(""),
        owned_line("  NAIC quarterly", "Due 2025-07-31", C_MUTED),
        owned_line("  EU AI Act transparency report", "Pending", C_AMBER),
        owned_line("  Board pack", "Ready for export", C_GREEN),
        owned_line("  Actuarial opinion", "Drafted", C_BLUE),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Use e to open the export workflow.",
            Style::default().fg(C_DIM),
        )]),
    ];
    if let Some(export) = app.last_export_for(ViewId::Reporting) {
        lines.push(Line::from(""));
        lines.push(owned_line(
            "  Last export",
            &format!(
                "{} for {} at {}",
                export.format, export.audience, export.created_at
            ),
            C_CYAN,
        ));
    }
    f.render_widget(Paragraph::new(lines), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_live_logs(f: &mut Frame, app: &App, area: Rect) {
    let shell = panel_shell(f, area, "LIVE SYSTEM LOGS - STREAMING");
    let visible = shell.content.height as usize;
    let skip = app.logs.len().saturating_sub(visible);
    let lines = app
        .logs
        .iter()
        .skip(skip)
        .map(|log| {
            let (lvl_color, lvl_str) = match log.level {
                'I' => (C_BLUE, "[I]"),
                'W' => (C_AMBER, "[W]"),
                'E' => (C_RED, "[E]"),
                _ => (C_MUTED, "[D]"),
            };
            Line::from(vec![
                Span::styled(format!(" {} ", log.ts), Style::default().fg(C_DIM)),
                Span::styled(
                    lvl_str,
                    Style::default().fg(lvl_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(format!("{:<18}", log.source), Style::default().fg(C_CYAN)),
                Span::styled(log.msg.clone(), Style::default().fg(C_MUTED)),
            ])
        })
        .collect::<Vec<_>>();
    f.render_widget(Paragraph::new(lines), shell.content);
    render_hint(f, shell.hint, &app.hint_text_for(app.active_view()));
}

fn draw_static_panel(f: &mut Frame, area: Rect, title: &str, body: &[&str], hint: String) {
    let shell = panel_shell(f, area, title);
    let mut lines = vec![Line::from("")];
    for line in body {
        lines.push(Line::from(vec![Span::styled(
            format!("  {line}"),
            Style::default().fg(C_MUTED),
        )]));
    }
    f.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: false }),
        shell.content,
    );
    render_hint(f, shell.hint, &hint);
}

fn bar(pct: u16) -> String {
    let filled = (pct / 5) as usize;
    let empty = 20usize.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn format_money(amount: u32) -> String {
    let digits = amount.to_string();
    let mut reversed = String::new();
    for (index, ch) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            reversed.push(',');
        }
        reversed.push(ch);
    }
    format!("${}", reversed.chars().rev().collect::<String>())
}
