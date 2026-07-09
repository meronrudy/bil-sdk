use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::App,
    models::{NavEntry, Panel},
    theme::{C_AMBER, C_BG2, C_BG3, C_BLUE, C_BORDER, C_DIM, C_GREEN, C_MUTED, C_PURPLE},
};

pub fn draw_titlebar(f: &mut Frame, area: Rect) {
    let title =
        "  ◈ AXIOM  ·  AI Assurance Actuarial Engine  v0.9.2  ·  Press ? for contextual help";
    f.render_widget(
        Paragraph::new(title)
            .style(Style::default().fg(C_MUTED).bg(C_BG3))
            .alignment(Alignment::Left),
        area,
    );
}

pub fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(C_BORDER))
        .style(Style::default().bg(C_BG2));
    f.render_widget(block, area);

    let inner = Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(1),
        height: area.height,
    };

    let mut items = Vec::new();
    for (section_label, entries) in nav_sections() {
        items.push(
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    section_label,
                    Style::default().fg(C_DIM).add_modifier(Modifier::BOLD),
                ),
            ]))
            .style(Style::default().bg(C_BG2)),
        );
        for entry in entries {
            let active = entry.panel == app.active_panel;
            let mut spans = vec![
                Span::styled(
                    if active { "▌ " } else { "  " },
                    Style::default().fg(if active { C_BLUE } else { C_BG2 }),
                ),
                Span::styled(
                    entry.label,
                    Style::default().fg(if active { C_BLUE } else { C_MUTED }),
                ),
            ];
            if let Some((badge, color)) = entry.badge {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    badge,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            }
            items.push(ListItem::new(Line::from(spans)).style(if active {
                Style::default().bg(Color::Rgb(20, 35, 60))
            } else {
                Style::default().bg(C_BG2)
            }));
        }
        items.push(ListItem::new(Line::from("")).style(Style::default().bg(C_BG2)));
    }

    f.render_widget(List::new(items).style(Style::default().bg(C_BG2)), inner);
}

pub fn draw_statusbar(f: &mut Frame, app: &App, area: Rect) {
    let mut items = vec![
        Span::styled("  ● Engine Online ", Style::default().fg(C_GREEN)),
        Span::styled("│ ", Style::default().fg(C_BORDER)),
        Span::styled("● DB Connected ", Style::default().fg(C_BLUE)),
        Span::styled("│ ", Style::default().fg(C_BORDER)),
        Span::styled("⚠ 2 Bias Alerts ", Style::default().fg(C_AMBER)),
        Span::styled("│ ", Style::default().fg(C_BORDER)),
        Span::styled("⬡ 7 Models Live ", Style::default().fg(C_PURPLE)),
    ];
    if let Some(flash) = &app.flash {
        items.push(Span::styled("│ ", Style::default().fg(C_BORDER)));
        items.push(Span::styled(
            format!("{} ", flash.text),
            Style::default().fg(flash.tone.color()),
        ));
    }

    let left_line = Line::from(items);
    let right_line = Line::from(vec![Span::styled(
        format!("{}  ", app.clock_str),
        Style::default().fg(C_DIM),
    )]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(26)])
        .split(area);
    f.render_widget(
        Paragraph::new(left_line).style(Style::default().bg(C_BG3)),
        chunks[0],
    );
    f.render_widget(
        Paragraph::new(right_line)
            .style(Style::default().bg(C_BG3))
            .alignment(Alignment::Right),
        chunks[1],
    );
}

fn nav_sections() -> Vec<(&'static str, Vec<NavEntry>)> {
    vec![
        (
            "OVERVIEW",
            vec![
                NavEntry {
                    label: "◈  Dashboard",
                    panel: Panel::Dashboard,
                    badge: None,
                },
                NavEntry {
                    label: "⬡  Risk Registry",
                    panel: Panel::RiskRegistry,
                    badge: Some(("12", C_AMBER)),
                },
            ],
        ),
        (
            "UNDERWRITING",
            vec![
                NavEntry {
                    label: "▦  Submissions",
                    panel: Panel::Submissions,
                    badge: Some(("3", C_AMBER)),
                },
                NavEntry {
                    label: "☰  Policy Config",
                    panel: Panel::PolicyConfig,
                    badge: None,
                },
                NavEntry {
                    label: "$  Quote Engine",
                    panel: Panel::QuoteEngine,
                    badge: None,
                },
            ],
        ),
        (
            "ACTUARIAL",
            vec![
                NavEntry {
                    label: "∿  Loss Models",
                    panel: Panel::LossModels,
                    badge: None,
                },
                NavEntry {
                    label: "⬕  Exposure",
                    panel: Panel::ExposureAnalysis,
                    badge: None,
                },
                NavEntry {
                    label: "⊞  Reserves",
                    panel: Panel::Reserves,
                    badge: Some(("OK", C_GREEN)),
                },
                NavEntry {
                    label: "↺  Retrospective",
                    panel: Panel::Retrospective,
                    badge: None,
                },
            ],
        ),
        (
            "AI ASSURANCE",
            vec![
                NavEntry {
                    label: "◎  Model Registry",
                    panel: Panel::ModelRegistry,
                    badge: None,
                },
                NavEntry {
                    label: "⚖  Bias Monitor",
                    panel: Panel::BiasMonitor,
                    badge: Some(("2", C_AMBER)),
                },
                NavEntry {
                    label: "〜  Drift Detection",
                    panel: Panel::DriftDetection,
                    badge: None,
                },
                NavEntry {
                    label: "⊙  Explainability",
                    panel: Panel::Explainability,
                    badge: None,
                },
                NavEntry {
                    label: "✎  Audit Trail",
                    panel: Panel::AuditTrail,
                    badge: None,
                },
            ],
        ),
        (
            "REGULATORY",
            vec![
                NavEntry {
                    label: "✓  Compliance",
                    panel: Panel::Compliance,
                    badge: Some(("OK", C_GREEN)),
                },
                NavEntry {
                    label: "≡  Reporting",
                    panel: Panel::Reporting,
                    badge: None,
                },
            ],
        ),
        (
            "SYSTEM",
            vec![
                NavEntry {
                    label: "⚙  Config",
                    panel: Panel::Config,
                    badge: None,
                },
                NavEntry {
                    label: "▸  Live Logs",
                    panel: Panel::LiveLogs,
                    badge: None,
                },
            ],
        ),
    ]
}
