use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    app::App,
    models::{ActionKey, UiMode},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Quit,
    OpenHelp,
    CloseHelp,
    NavDown,
    NavUp,
    MoveRow(i32),
    NextTab,
    PrevTab,
    RequestAction(ActionKey),
    CommitFilter,
    CancelFilter,
    FilterBackspace,
    FilterChar(char),
    CloseDrawer,
    DrawerFocusNext,
    DrawerFocusPrev,
    DrawerAdjust(i32),
    DrawerSubmit,
    Tick,
}

pub fn command_from_key(app: &App, key: KeyEvent) -> Option<Command> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match app.ui_mode {
        UiMode::Normal => command_from_normal_mode(app, key),
        UiMode::Help => command_from_help_mode(key),
        UiMode::Filter => command_from_filter_mode(key),
        UiMode::Drawer => command_from_drawer_mode(app, key),
    }
}

pub fn apply_command(app: &mut App, command: Command) {
    match command {
        Command::Quit => app.set_should_quit(),
        Command::OpenHelp => app.open_help(),
        Command::CloseHelp => app.close_help(),
        Command::NavDown => app.nav_down(),
        Command::NavUp => app.nav_up(),
        Command::MoveRow(delta) => app.move_row(delta),
        Command::NextTab => app.next_tab(),
        Command::PrevTab => app.prev_tab(),
        Command::RequestAction(action) => app.handle_action_request(action),
        Command::CommitFilter => app.commit_filter(),
        Command::CancelFilter => app.cancel_filter(),
        Command::FilterBackspace => app.pop_filter_char(),
        Command::FilterChar(ch) => app.push_filter_char(ch),
        Command::CloseDrawer => {
            app.close_drawer();
            app.flash = Some(crate::models::FlashMessage {
                text: "Drawer closed".to_string(),
                tone: crate::models::FlashTone::Info,
                ticks_left: 40,
            });
        }
        Command::DrawerFocusNext => app.focus_next_in_drawer(),
        Command::DrawerFocusPrev => app.focus_prev_in_drawer(),
        Command::DrawerAdjust(delta) => app.adjust_drawer(delta),
        Command::DrawerSubmit => app.submit_drawer(),
        Command::Tick => app.tick(),
    }
}

fn command_from_normal_mode(app: &App, key: KeyEvent) -> Option<Command> {
    let view = app.active_view();
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(Command::Quit),
        KeyCode::Char('?') => Some(Command::OpenHelp),
        KeyCode::Char('J') => Some(Command::NavDown),
        KeyCode::Char('K') => Some(Command::NavUp),
        KeyCode::Char('j') | KeyCode::Down => {
            if app.supports_rows(view) && app.visible_row_count() > 0 {
                Some(Command::MoveRow(1))
            } else {
                Some(Command::NavDown)
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.supports_rows(view) && app.visible_row_count() > 0 {
                Some(Command::MoveRow(-1))
            } else {
                Some(Command::NavUp)
            }
        }
        KeyCode::Tab => Some(Command::NextTab),
        KeyCode::BackTab => Some(Command::PrevTab),
        KeyCode::Enter => Some(Command::RequestAction(ActionKey::Inspect)),
        KeyCode::Char('a') => Some(Command::RequestAction(ActionKey::Assign)),
        KeyCode::Char('r') => Some(Command::RequestAction(ActionKey::Refer)),
        KeyCode::Char('e') => Some(Command::RequestAction(ActionKey::Export)),
        KeyCode::Char('s') => Some(Command::RequestAction(ActionKey::Simulate)),
        KeyCode::Char('/') => Some(Command::RequestAction(ActionKey::Filter)),
        _ => None,
    }
}

fn command_from_help_mode(key: KeyEvent) -> Option<Command> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(Command::Quit),
        KeyCode::Esc | KeyCode::Char('?') => Some(Command::CloseHelp),
        _ => None,
    }
}

fn command_from_filter_mode(key: KeyEvent) -> Option<Command> {
    match key.code {
        KeyCode::Esc => Some(Command::CancelFilter),
        KeyCode::Enter => Some(Command::CommitFilter),
        KeyCode::Backspace => Some(Command::FilterBackspace),
        KeyCode::Char(ch)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            Some(Command::FilterChar(ch))
        }
        _ => None,
    }
}

fn command_from_drawer_mode(app: &App, key: KeyEvent) -> Option<Command> {
    let drawer = app.drawer_state.as_ref()?;
    match key.code {
        KeyCode::Esc => Some(Command::CloseDrawer),
        KeyCode::Tab => Some(Command::DrawerFocusNext),
        KeyCode::BackTab => Some(Command::DrawerFocusPrev),
        KeyCode::Char('j') | KeyCode::Down => Some(Command::DrawerAdjust(1)),
        KeyCode::Char('k') | KeyCode::Up => Some(Command::DrawerAdjust(-1)),
        KeyCode::Enter => {
            if drawer.is_detail() {
                Some(Command::CloseDrawer)
            } else if drawer.focus < drawer.field_count() - 1 {
                Some(Command::DrawerFocusNext)
            } else {
                Some(Command::DrawerSubmit)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_translation_emits_expected_actions() {
        let mut app = App::default();
        assert_eq!(
            command_from_key(&app, KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE)),
            Some(Command::OpenHelp)
        );
        assert_eq!(
            command_from_key(&app, KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE)),
            Some(Command::NavDown)
        );

        app.start_filter();
        assert_eq!(
            command_from_key(&app, KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE)),
            Some(Command::FilterChar('x'))
        );
        assert_eq!(
            command_from_key(&app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            Some(Command::CommitFilter)
        );

        app.close_help();
        app.open_drawer(crate::models::DrawerState::detail(
            crate::models::DetailTarget::Risk(0),
        ));
        assert_eq!(
            command_from_key(&app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            Some(Command::CloseDrawer)
        );
    }
}
