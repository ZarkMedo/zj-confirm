# zj-confirm 插件开发摘要

## 项目背景

**目标：** 开发一个 Zellij 插件，在关闭 Pane/Tab 时弹出确认对话框，防止误操作。

**原项目：** 基于 [dj95/zj-quit](https://github.com/dj95/zj-quit) 修改，该插件只支持 Session 关闭确认。

**仓库路径：** `/Users/medozark/code/project/gitPro/tools/plugins/zellij/zj-confirm`

---

## 需求分析

| 操作 | 确认插件 | 状态 |
|------|---------|------|
| 关闭 Session | zj-quit | ✅ 已有 |
| 关闭 Pane | 无 | ❌ 需开发 |
| 关闭 Tab | 无 | ❌ 需开发 |

---

## 技术方案

### Zellij 插件架构

- **语言：** Rust + WebAssembly (WASI)
- **SDK：** `zellij-tile`
- **编译输出：** `.wasm` 文件

### 关键 API (PR #3576)

```rust
// 关闭 Tab
CloseTabWithIndex(tab_index: u32)

// 关闭 Pane
ClosePaneWithId(pane_id: u32)
```

### 项目结构

```
zj-confirm/
├── Cargo.toml          # 依赖配置
├── src/lib.rs          # 核心逻辑
└── README.md
```

### 核心依赖

```toml
[dependencies]
zellij-tile = "0.41.1"
```

---

## 实现思路

1. **UI 层：** 显示确认对话框（类似 zj-quit 的浮动窗口）
2. **按键处理：** 监听确认键/取消键
3. **API 调用：** 用户确认后调用 `ClosePaneWithId` 或 `CloseTabWithIndex`

---

## 配置使用

在 `~/.config/zellij/config.kdl` 中：

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

---

## 待完成工作

- [ ] 修改 `src/lib.rs` 实现 Pane 关闭确认
- [ ] 修改 `src/lib.rs` 实现 Tab 关闭确认
- [ ] 修改项目名称为 zj-confirm
- [ ] 编译为 WASM
- [ ] 测试验证

---

## 参考资料

- [Zellij 插件开发文档](https://zellij.dev/documentation/plugins.html)
- [zj-quit 源码](https://github.com/dj95/zj-quit)
- [zellij-tile API](https://github.com/zellij-org/zellij/tree/main/zellij-tile)
- [PR #3576 - Plugin APIs to affect other panes](https://github.com/zellij-org/zellij/pull/3576)

---

## 对话记录时间

生成时间：2026-04-05 21:27:37 CST
