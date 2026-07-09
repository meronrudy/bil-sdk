use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::App,
    fixtures::{
        ASSIGNEES, EXPORT_AUDIENCES, REFERRAL_REASONS, REFERRAL_TARGETS, SIM_ITERATIONS, URGENCIES,
    },
    models::{
        available_actions, export_formats, simulation_scenarios, view_name, AssignContext,
        DrawerKind, ReferContext,
    },
    theme::{C_BG3, C_BLUE, C_CYAN, C_DIM, C_GREEN, C_MUTED},
};

use super::layout::centered_rect;

pub fn draw_filter_bar(f: &mut Frame, app: &App, area: Rect) {
    let width = area.width.saturating_sub(4).max(10);
    let bar_area = Rect {
        x: area.x + 2,
        y: area.y + area.height.saturating_sub(2),
        width,
        height: 1,
    };
    f.render_widget(Clear, bar_area);
    let text = Line::from(vec![
        Span::styled(" / ", Style::default().fg(C_BLUE).bg(C_BG3)),
        Span::styled(
            format!(
                "Filter {}: {}",
                view_name(app.filter_state.view),
                if app.filter_state.input.is_empty() {
                    "type to search".to_string()
                } else {
                    app.filter_state.input.clone()
                }
            ),
            Style::default().fg(crate::theme::C_TEXT).bg(C_BG3),
        ),
        Span::styled(
            format!(
                "  {} matches  Enter apply  Esc cancel",
                app.visible_row_count_for(app.filter_state.view)
            ),
            Style::default().fg(C_DIM).bg(C_BG3),
        ),
    ]);
    f.render_widget(
        Paragraph::new(text).style(Style::default().bg(C_BG3)),
        bar_area,
    );
}

pub fn draw_help_popup(f: &mut Frame, app: &App, area: Rect) {
    let actions = available_actions(app.active_view());
    let popup_h = (14 + actions.len() as u16).min(area.height.saturating_sub(2));
    let popup = centered_rect(area, 64, popup_h);
    f.render_widget(Clear, popup);
    let block = Block::default()
        .title(format!(" KEYBINDINGS - {} ", view_name(app.active_view())))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(C_CYAN))
        .style(Style::default().bg(crate::theme::C_BG2));
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("  GLOBAL", Style::default().fg(C_CYAN))]),
        kb_line("J / K", "Next / previous panel"),
        kb_line("j / k", "Move visible row when the current view has rows"),
        kb_line("Tab", "Next tab within panel"),
        kb_line("Shift+Tab", "Previous tab within panel"),
        kb_line("?", "Close this help"),
        kb_line("Esc", "Close help / drawer / filter"),
        kb_line("q", "Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  CURRENT VIEW ACTIONS",
            Style::default().fg(C_CYAN),
        )]),
    ];
    if actions.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "  No contextual actions on this view.",
            Style::default().fg(C_DIM),
        )]));
    } else {
        for action in actions {
            lines.push(kb_line(action.key_label(), action.help_text()));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Unsupported action keys show a status message instead of failing silently.",
        Style::default().fg(C_DIM),
    )]));
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

pub fn draw_drawer(f: &mut Frame, app: &App, area: Rect) {
    let Some(drawer) = &app.drawer_state else {
        return;
    };

    let drawer_area = if area.width >= 120 {
        Rect {
            x: area.x + area.width.saturating_sub(44),
            y: area.y + 1,
            width: 42,
            height: area.height.saturating_sub(2),
        }
    } else {
        centered_rect(
            area,
            area.width.saturating_sub(6).min(72),
            area.height.saturating_sub(6).min(24),
        )
    };

    f.render_widget(Clear, drawer_area);
    let title = match &drawer.kind {
        DrawerKind::Detail(target) => app.detail_title(*target),
        DrawerKind::Assign { context, .. } => match context {
            AssignContext::Submission(idx) => {
                format!("ASSIGN - {}", app.submissions[*idx].ref_id)
            }
            AssignContext::Quote => "ASSIGN - QUOTE ENGINE".to_string(),
        },
        DrawerKind::Refer { context, .. } => match context {
            ReferContext::Submission(idx) => {
                format!("REFER - {}", app.submissions[*idx].ref_id)
            }
            ReferContext::Quote => "REFER - QUOTE ENGINE".to_string(),
        },
        DrawerKind::Export { view, .. } => format!("EXPORT - {}", view_name(*view)),
        DrawerKind::Simulate { target, .. } => format!("SIMULATE - {}", target.label()),
    };
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(C_BLUE))
        .style(Style::default().bg(crate::theme::C_BG2));
    let inner = block.inner(drawer_area);
    f.render_widget(block, drawer_area);

    let content = match &drawer.kind {
        DrawerKind::Detail(target) => app.detail_lines(*target),
        DrawerKind::Assign {
            assignee,
            urgency,
            button,
            ..
        } => vec![
            Line::from(""),
            field_value_line("Assignee", ASSIGNEES[*assignee], drawer.focus == 0),
            option_line("Choices", ASSIGNEES, *assignee, drawer.focus == 0),
            Line::from(""),
            field_value_line("Urgency", URGENCIES[*urgency], drawer.focus == 1),
            option_line("Levels", URGENCIES, *urgency, drawer.focus == 1),
            Line::from(""),
            button_line(*button, drawer.focus == 2),
            Line::from(""),
            Line::from(vec![Span::styled(
                "  Tab cycles fields. j/k changes the focused option. Enter confirms on the action row.",
                Style::default().fg(C_DIM),
            )]),
        ],
        DrawerKind::Refer {
            reason,
            target,
            button,
            ..
        } => vec![
            Line::from(""),
            field_value_line("Reason", REFERRAL_REASONS[*reason], drawer.focus == 0),
            option_line("Reasons", REFERRAL_REASONS, *reason, drawer.focus == 0),
            Line::from(""),
            field_value_line("Target", REFERRAL_TARGETS[*target], drawer.focus == 1),
            option_line("Targets", REFERRAL_TARGETS, *target, drawer.focus == 1),
            Line::from(""),
            button_line(*button, drawer.focus == 2),
        ],
        DrawerKind::Export {
            view,
            format,
            audience,
            button,
        } => vec![
            Line::from(""),
            field_value_line("Format", export_formats(*view)[*format], drawer.focus == 0),
            option_line("Formats", export_formats(*view), *format, drawer.focus == 0),
            Line::from(""),
            field_value_line("Audience", EXPORT_AUDIENCES[*audience], drawer.focus == 1),
            option_line("Audience", EXPORT_AUDIENCES, *audience, drawer.focus == 1),
            Line::from(""),
            button_line(*button, drawer.focus == 2),
        ],
        DrawerKind::Simulate {
            target,
            scenario,
            iterations,
            button,
        } => vec![
            Line::from(""),
            field_value_line(
                "Scenario",
                simulation_scenarios(*target)[*scenario],
                drawer.focus == 0,
            ),
            option_line(
                "Presets",
                simulation_scenarios(*target),
                *scenario,
                drawer.focus == 0,
            ),
            Line::from(""),
            field_value_line(
                "Iterations",
                &SIM_ITERATIONS[*iterations].to_string(),
                drawer.focus == 1,
            ),
            option_line_owned(
                "Counts",
                &SIM_ITERATIONS
                    .iter()
                    .map(|count| count.to_string())
                    .collect::<Vec<_>>(),
                *iterations,
                drawer.focus == 1,
            ),
            Line::from(""),
            button_line(*button, drawer.focus == 2),
        ],
    };
    f.render_widget(Paragraph::new(content).wrap(Wrap { trim: false }), inner);
}

fn kb_line(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  {key:<14}"), Style::default().fg(C_BLUE)),
        Span::styled(desc.to_string(), Style::default().fg(C_MUTED)),
    ])
}

fn field_value_line(label: &str, value: &str, focused: bool) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {:<12}", label),
            Style::default().fg(if focused { C_BLUE } else { C_MUTED }),
        ),
        Span::styled(
            value.to_string(),
            Style::default()
                .fg(crate::theme::C_TEXT)
                .add_modifier(if focused {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
    ])
}

fn option_line(label: &str, options: &[&str], selected: usize, focused: bool) -> Line<'static> {
    let mut spans = vec![Span::styled(
        format!("  {:<12}", label),
        Style::default().fg(C_DIM),
    )];
    for (index, option) in options.iter().enumerate() {
        let active = index == selected;
        spans.push(Span::styled(
            format!("[{}]", option),
            Style::default()
                .fg(if active {
                    if focused {
                        C_BLUE
                    } else {
                        C_CYAN
                    }
                } else {
                    C_MUTED
                })
                .add_modifier(if active {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ));
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn option_line_owned(
    label: &str,
    options: &[String],
    selected: usize,
    focused: bool,
) -> Line<'static> {
    let mut spans = vec![Span::styled(
        format!("  {:<12}", label),
        Style::default().fg(C_DIM),
    )];
    for (index, option) in options.iter().enumerate() {
        let active = index == selected;
        spans.push(Span::styled(
            format!("[{}]", option),
            Style::default()
                .fg(if active {
                    if focused {
                        C_BLUE
                    } else {
                        C_CYAN
                    }
                } else {
                    C_MUTED
                })
                .add_modifier(if active {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ));
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn button_line(selected: usize, focused: bool) -> Line<'static> {
    let confirm_style = Style::default()
        .fg(if selected == 0 { C_GREEN } else { C_MUTED })
        .add_modifier(if selected == 0 || focused {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });
    let cancel_style = Style::default()
        .fg(if selected == 1 {
            crate::theme::C_AMBER
        } else {
            C_MUTED
        })
        .add_modifier(if selected == 1 || focused {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });
    Line::from(vec![
        Span::styled("  Action       ", Style::default().fg(C_DIM)),
        Span::styled("[Confirm]", confirm_style),
        Span::raw("  "),
        Span::styled("[Cancel]", cancel_style),
    ])
}
