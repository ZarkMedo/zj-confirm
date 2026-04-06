# zj-confirm Implementation Plan

**Goal:** 开发一个 Zellij 插件，在关闭 Session/Pane/Tab 时弹出确认对话框，防止误操作。

**Architecture:** 使用状态机 (Menu → Confirming → 执行/隐藏)，分支菜单选择操作类型，确认后调用对应 API。

**Tech Stack:** Rust + WebAssembly (WASI), zellij-tile 0.41.0

---

## 文件清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `Cargo.toml` | ✅ 已完成 | name: "zj-confirm", version: "0.0.1", authors: "zarkmedo" |
| `src/main.rs` | 重写 | 状态机、菜单渲染、按键处理、API 调用 |
| `README.md` | 修改 | 更新使用说明 |

---

## 实现任务

### Task 1: 重写 src/main.rs

**文件:**
- 修改: `src/main.rs` (完整重写)

- [ ] **Step 1: 定义状态枚举和 State 结构体**

```rust
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
                        KeyWithModifier { modifier: Modifier::None, key: BareKey::Char('s') } => {
                            self.current_state = ConfirmState::ConfirmSession;
                        }
                        KeyWithModifier { modifier: Modifier::None, key: BareKey::Char('p') } => {
                            self.current_state = ConfirmState::ConfirmPane;
                        }
                        KeyWithModifier { modifier: Modifier::None, key: BareKey::Char('t') } => {
                            self.current_state = ConfirmState::ConfirmTab;
                        }
                        KeyWithModifier { modifier: Modifier::None, key: BareKey::Char('f') } => {
                            self.current_state = ConfirmState::ConfirmForcePane;
                        }
                        _ if self.cancel_key == key => hide_self(),
                    }
                } else {
                    match key {
                        KeyWithModifier { modifier: Modifier::None, key: BareKey::Char('y') } => {
                            self.execute_action();
                            hide_self();
                        }
                        _ if self.cancel_key == key || key == KeyWithModifier::new(BareKey::Char('n')) => {
                            self.current_state = ConfirmState::Menu;
                        }
                        _ if key == KeyWithModifier::new(BareKey::Char('e')) => {
                            hide_self();
                        }
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
            "[P] Pane      - Close current pane",
            "[T] Tab       - Close current tab",
            "[F] Force     - Force close current pane",
            "[E] Escape    - Cancel/Hide",
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
                // ClosePaneWithId 需要 pane_id
                // TODO: 实现 pane 关闭
            }
            ConfirmState::ConfirmTab => {
                // CloseTabWithIndex 需要 tab_index
                // TODO: 实现 tab 关闭
            }
            ConfirmState::ConfirmForcePane => {
                // KillPane
                // TODO: 实现强制关闭 pane
            }
            _ => {}
        }
    }
}
```

---

### Task 2: 更新 README.md

**文件:**
- 修改: `README.md`

- [ ] **Step 1: 更新 README 内容**

更新文档以反映新的插件名称和功能。

---

### Task 3: 编译测试

**文件:**
- 无

- [ ] **Step 1: 编译 WASM**

```bash
cd /Users/medozark/code/project/gitPro/tools/plugins/zellij/zj-confirm
cargo build --release
```

预期输出: `target/wasm32-wasi/release/zj-confirm.wasm`

---

## 自检清单

- [ ] Spec 覆盖: 每个设计需求都有对应任务
- [ ] 无占位符: 无 "TBD", "TODO", "实现 later"
- [ ] 类型一致性: 所有方法签名和类型在整个计划中一致
- [ ] 文件路径: 所有路径准确
