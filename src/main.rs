use zellij_tile::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq)]
enum ConfirmState {
    Menu,
    ConfirmSession,
    ConfirmPane,
    ConfirmTab,
    ConfirmForcePane,
}

struct State {
    current_state: ConfirmState,
    confirm_key: KeyWithModifier,
    cancel_key: KeyWithModifier,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_state: ConfirmState::Menu,
            confirm_key: KeyWithModifier::new(BareKey::Enter),
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
                if self.current_state == ConfirmState::Menu {
                    match key {
                        KeyWithModifier { bare_key: BareKey::Char('s'), key_modifiers } if key_modifiers.is_empty() => {
                            self.current_state = ConfirmState::ConfirmSession;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('p'), key_modifiers } if key_modifiers.is_empty() => {
                            self.current_state = ConfirmState::ConfirmPane;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('t'), key_modifiers } if key_modifiers.is_empty() => {
                            self.current_state = ConfirmState::ConfirmTab;
                        }
                        KeyWithModifier { bare_key: BareKey::Char('f'), key_modifiers } if key_modifiers.is_empty() => {
                            self.current_state = ConfirmState::ConfirmForcePane;
                        }
                        _ if self.cancel_key == key => hide_self(),
                        _ => {}
                    }
                } else {
                    match key {
                        KeyWithModifier { bare_key: BareKey::Char('y'), key_modifiers } if key_modifiers.is_empty() => {
                            self.execute_action();
                            hide_self();
                        }
                        _ if self.cancel_key == key || key == KeyWithModifier::new(BareKey::Char('n')) => {
                            self.current_state = ConfirmState::Menu;
                        }
                        _ if key == KeyWithModifier::new(BareKey::Char('e')) => {
                            hide_self();
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
            | ConfirmState::ConfirmTab
            | ConfirmState::ConfirmForcePane => self.render_confirm(rows, cols),
        }
    }
}

impl State {
    fn render_menu(&self, rows: usize, cols: usize) {
        let items = vec![
            "[S] Session   - Close entire session",
            "[P] Pane     - Close current pane",
            "[T] Tab      - Close current tab",
            "[F] Force    - Force close current pane",
            "[E] Escape   - Cancel/Hide",
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
            ConfirmState::ConfirmForcePane => "Force Close Pane?",
            _ => return,
        };

        let confirm_text = format!("{} [Y] Yes  [N] No  [E] Escape", msg);
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
                // TODO: 实现 pane 关闭 - 使用 ClosePaneWithId
            }
            ConfirmState::ConfirmTab => {
                // TODO: 实现 tab 关闭 - 使用 CloseTabWithIndex
            }
            ConfirmState::ConfirmForcePane => {
                // TODO: 实现强制关闭 pane - 使用 KillPane
            }
            _ => {}
        }
    }
}
