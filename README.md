# zj-confirm

A [zellij](https://zellij.dev/) plugin that shows a confirmation dialog before closing Session, Pane, or Tab.

## Features

- [S] Session - Close entire Zellij session
- [P] Pane - Close current pane
- [T] Tab - Close current tab
- [F] Force - Force close current pane

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
    alt-q "TogglePlugin" "zj-confirm"
  }
}
```

## Usage

Press `alt-q` to open the confirmation menu, then:
- Press `S` to close session
- Press `P` to close pane
- Press `T` to close tab
- Press `F` to force close pane
- Press `E` or `Esc` to cancel
