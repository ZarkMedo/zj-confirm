# zj-confirm 设计文档

## 概述

Zellij 插件，在关闭 Session/Pane/Tab 时弹出确认对话框，防止误操作。

## 交互流程

```
用户按快捷键激活插件
        ↓
   ┌─────────────────────────────┐
   │      zj-confirm 菜单         │
   │                             │
   │  [S] Session   - 关闭整个 session  │
   │  [P] Pane      - 关闭当前 pane      │
   │  [T] Tab       - 关闭当前 tab       │
   │  [F] Force     - 强制关闭当前 pane   │
   │                             │
   │  [E] Escape    - 取消/隐藏         │
   └─────────────────────────────┘
        ↓
   用户按 S/P/T/F/E
        ↓
   ┌─────────────────────────────┐
   │      确认对话框               │
   │                             │
   │  "Close Session? Y/n"       │
   │  "Close Pane? Y/n"          │
   │  "Close Tab? Y/n"           │
   │  "Force Close Pane? Y/n"     │
   └─────────────────────────────┘
        ↓
   按 Y 确认执行 / 按 n 或 E 取消
```

## 状态机

```
States: Menu → Confirming → (执行/隐藏)
```

| 状态 | 接收按键 | 动作 |
|------|---------|------|
| Menu | S/P/T/F | 进入对应 Confirming |
| Menu | E | 隐藏插件 |
| Confirming | Y | 执行操作 |
| Confirming | N/E | 返回 Menu |

## API

| 操作 | API |
|------|-----|
| Session | `quit_zellij()` |
| Pane | `ClosePaneWithId(pane_id)` |
| Tab | `CloseTabWithIndex(tab_index)` |
| Force Pane | `KillPane` |

## 项目结构

```
zj-confirm/
├── Cargo.toml          # name = "zj-confirm"
├── src/
│   └── main.rs         # 插件逻辑
└── README.md
```

## 配置示例 (config.kdl)

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

## 实现任务

1. 修改 Cargo.toml: name = "zj-confirm"
2. 修改 src/main.rs:
   - 新增 State 结构体的状态字段 (Menu/Confirming)
   - 实现菜单渲染
   - 实现确认对话框渲染
   - 实现按键处理逻辑
   - 实现各操作的 API 调用
3. 更新 README.md
4. 编译为 WASM
5. 测试验证
