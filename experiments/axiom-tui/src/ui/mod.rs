pub mod chrome;
pub mod layout;
pub mod overlays;
pub mod panels;
pub mod tables;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
    Frame,
};

use crate::{app::App, models::UiMode, theme::C_BG};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    chrome::draw_titlebar(f, root[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(0)])
        .split(root[1]);

    chrome::draw_sidebar(f, app, body[0]);
    panels::draw_main(f, app, body[1]);
    chrome::draw_statusbar(f, app, root[2]);

    match app.ui_mode {
        UiMode::Filter => overlays::draw_filter_bar(f, app, body[1]),
        UiMode::Help => overlays::draw_help_popup(f, app, area),
        UiMode::Drawer => overlays::draw_drawer(f, app, body[1]),
        UiMode::Normal => {}
    }
}
