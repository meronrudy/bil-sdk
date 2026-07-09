use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, TableState},
    Frame,
};

use crate::{
    app::App,
    models::ViewId,
    theme::{C_BG, C_BG2, C_BORDER, C_DIM},
};

pub struct PanelShell {
    pub content: Rect,
    pub hint: Rect,
}

pub fn panel_shell(f: &mut Frame, area: Rect, title: &str) -> PanelShell {
    let block = panel_block(title);
    let inner = block.inner(area);
    f.render_widget(block, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);
    PanelShell {
        content: chunks[0],
        hint: chunks[1],
    }
}

pub fn render_hint(f: &mut Frame, area: Rect, text: &str) {
    if area.height == 0 {
        return;
    }
    f.render_widget(
        Paragraph::new(text.to_string()).style(Style::default().fg(C_DIM)),
        area,
    );
}

pub fn render_empty_state(f: &mut Frame, count: usize, area: Rect) {
    if count > 0 || area.width < 10 || area.height < 3 {
        return;
    }
    let msg_area = centered_rect(area, area.width.min(32), 3);
    f.render_widget(Clear, msg_area);
    f.render_widget(
        Paragraph::new("No rows match the current filter.")
            .style(Style::default().fg(C_DIM).bg(C_BG2))
            .alignment(Alignment::Center),
        msg_area,
    );
}

pub fn panel_block(title: &str) -> Block<'_> {
    Block::default()
        .title(format!(" {title} "))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(C_BORDER))
        .style(Style::default().bg(C_BG))
}

pub fn table_state_for(app: &App, view: ViewId) -> TableState {
    let mut state = TableState::default();
    state.select(app.selected_visible_position(view));
    state
}

pub fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width.saturating_sub(2)).max(4);
    let height = height.min(area.height.saturating_sub(2)).max(4);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}
