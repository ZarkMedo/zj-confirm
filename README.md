# zj-confirm

A [zellij](https://zellij.dev/) plugin that shows a confirmation dialog before closing Session or Tab.

## Features

- [S/s] Session - Close entire Zellij session
- [T/t] Tab - Close current tab
- [E/e] Escape - Cancel

## Why only Session and Tab?

### Pane close is not implemented

Zellij's plugin API (`zellij-tile`) does not provide a reliable way to close a specific pane by ID. The `close_focus()` function only closes the focused pane, which can lead to unexpected behavior when the plugin loses keyboard focus.

### Pane误操作成本偏低

Pane 的误操作成本相对较低：
- Pane 关闭后可以轻松重新创建 (`Ctrl+p n`)
- 可以通过 `Ctrl+p p` 快速切换回上一个 pane
-Pane 内容通常可以在其他地方恢复

相比之下，Session 和 Tab 的关闭会导致更严重的后果：
- Session 关闭会丢失所有 pane 和 tab
- Tab 关闭会丢失该 tab 内的所有 pane

因此，**Pane 关闭不需要确认**，而 Session 和 Tab 需要。

## Installation

1. Build the plugin:
```bash
cargo build --release --target wasm32-wasip1
```

2. Copy the WASM file to your desired location

3. Add to your `~/.config/zellij/config.kdl`:

```kdl
plugins {
  zj-confirm location="file:/path/to/zj-confirm.wasm"
}

keybinds {
  shared {
    alt-q "LaunchOrFocusPlugin" "zj-confirm" {
      floating true
    }
  }
}
```

## Usage

Press `alt-q` to open the confirmation menu, then:
- Press `S` or `s` to close session
- Press `T` or `t` to close tab
- Press `E`, `e` or `Esc` to cancel
