use zellij_tile::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq)]
enum ConfirmState {
    Menu,
    ConfirmSession,
    ConfirmPane,
    ConfirmTab,
}

struct State {
    current_state: ConfirmState,
    cancel_key: KeyWithModifier,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_state: ConfirmState::Menu,
            cancel_key: KeyWithModifier::new(BareKey::Esc),
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        subscribe(&[EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => {
                subscribe(&[EventType::Key]);

                if self.current_state == ConfirmState::Menu {
                    match key {
                        KeyWithModifier { bare_key: BareKey::Char('s'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('S'), .. } => {
                            self.current_state = ConfirmState::ConfirmSession;
                            return true;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('p'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('P'), .. } => {
                            self.current_state = ConfirmState::ConfirmPane;
                            return true;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('t'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('T'), .. } => {
                            self.current_state = ConfirmState::ConfirmTab;
                            return true;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('e'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('E'), .. } => {
                            hide_self();
                            return true;
                        }
                        _ if self.cancel_key == key => {
                            hide_self();
                            return true;
                        }
                        _ => {}
                    }
                } else {
                    match key {
                        KeyWithModifier { bare_key: BareKey::Char('y'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('Y'), .. } => {
                            self.execute_action();
                            hide_self();
                            return true;
                        }
                        _ if self.cancel_key == key => {
                            self.current_state = ConfirmState::Menu;
                            return true;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('n'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('N'), .. } => {
                            self.current_state = ConfirmState::Menu;
                            return true;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('e'), .. }
                        | KeyWithModifier { bare_key: BareKey::Char('E'), .. } => {
                            hide_self();
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        match self.current_state {
            ConfirmState::Menu => self.render_menu(rows, cols),
            ConfirmState::ConfirmSession
            | ConfirmState::ConfirmPane
            | ConfirmState::ConfirmTab => self.render_confirm(rows, cols),
        }
    }
}

impl State {
    fn render_menu(&self, rows: usize, cols: usize) {
        let items = vec![
            "[S/s] Session - Close entire session",
            "[P/p] Pane   - Close current pane",
            "[T/t] Tab    - Close current tab",
            "[E/e] Escape - Cancel/Hide",
        ];

        let start_y = (rows / 2) - (items.len() / 2);

        for (i, item) in items.iter().enumerate() {
            let y = start_y + i;
            let x = cols.saturating_sub(item.chars().count()) / 2;
            print_text_with_coordinates(
                Text::new(item.to_string()),
                x,
                y,
                None,
                None,
            );
        }
    }

    fn render_confirm(&self, rows: usize, cols: usize) {
        let msg = match self.current_state {
            ConfirmState::ConfirmSession => "Close Session?",
            ConfirmState::ConfirmPane => "Close Pane?",
            ConfirmState::ConfirmTab => "Close Tab?",
            _ => return,
        };

        let confirm_text = format!("{} [Y/y] Yes  [N/n] No  [E/e] Escape", msg);
        let y = rows / 2;
        let x = cols.saturating_sub(confirm_text.chars().count()) / 2;

        print_text_with_coordinates(
            Text::new(confirm_text),
            x,
            y,
            None,
            None,
        );
    }

    fn execute_action(&self) {
        match self.current_state {
            ConfirmState::ConfirmSession => quit_zellij(),
            ConfirmState::ConfirmPane => {
                close_focus();
            }
            ConfirmState::ConfirmTab => {
                close_focused_tab();
            }
            _ => {}
        }
    }
}
