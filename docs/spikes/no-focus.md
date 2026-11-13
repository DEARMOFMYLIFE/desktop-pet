# Spike · 宠物窗“不抢焦点”验证方案

> 日期：2026-11-09 · 状态：待实机验证（Linux 20.04 / WebKitGTK 4.0 无法运行 Tauri 2）
> 关联：`04-技术方案与工程规范.md` §4.2 / §7.2

## 目标

点击小人、拖动小人、弹出右键菜单时，**不抢走当前应用（编辑器/浏览器）的键盘焦点**。这是桌面宠物体验的硬要求，但跨平台 API 不统一。

## 当前实现

- `backend/tauri.conf.json` 的宠物窗设置了 `"focus": false`。
- 这只影响窗口创建时的初始焦点，**不等于**点击后不激活窗口。
- M0/M1 尚未在真实桌面环境验证；`cargo check` 只能保证配置合法。

## 平台方案

### Windows 10/11

- 目标窗口样式：`WS_EX_NOACTIVATE`（鼠标点击不激活、不进入 Alt+Tab）。
- 获取 HWND：
  - Tauri 2 `WebviewWindow` 实现 `raw_window_handle::HasWindowHandle`；
  - 在 `setup` 后调用 `window.window_handle()`，取 `RawWindowHandle::Win32(hwnd)`。
- 设置样式：
  ```rust
  let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
  SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | WS_EX_NOACTIVATE as isize);
  ```
- 需要新增 `windows` crate（`Win32_UI_WindowsAndMessaging` feature），仅 `#[cfg(target_os = "windows")]` 编译。
- 右键菜单弹出时可能短暂获得菜单焦点；关闭后应归还，需实测。

### macOS 12+

- 首选 `NSPanel` + `NSWindowStyleMask::NonactivatingPanel`，或重写 `canBecomeKeyWindow`/`canBecomeMainWindow` 返回 `false`。
- Tauri 默认 `NSWindow` 不支持非激活面板；需要 `objc2`/`cocoa` 平台扩展。
- 另一个方向：`ActivationPolicy::Accessory`（不显示 Dock 图标）+ `set_visible_on_all_workspaces(true)`；Accessory 只影响 Dock/菜单栏，不阻止点击激活。
- 需实测：点击宠物后，当前编辑器是否仍保持光标。

### Linux X11

- 窗口类型设为 `_NET_WM_WINDOW_TYPE_UTILITY` 或 `_NET_WM_WINDOW_TYPE_DOCK`，并设置 `WM_HINTS.input = False` 可让窗口不接受输入，但那样点击事件也收不到。
- 更实际的做法：保持普通窗口 + `focus: false` + `alwaysOnTop`，接受“点击时短暂聚焦”，或使用 `X11` `_NET_ACTIVE_WINDOW` 保护（复杂）。
- X11 下多数窗口管理器点击即聚焦，**不抢焦点很难完全做到**；产品上可用“点击穿透模式”规避。
- Wayland 下无统一方案，按降级处理。

## 建议决策

1. **M1 先做 Windows/macOS 平台 spike**，Linux 接受“点击聚焦”并强化点击穿透。
2. 若三端无法统一，产品文案改为“尽量不打断”，并在设置中提供“点击穿透”快捷键。
3. 平台扩展代码集中在 `backend/src/windows/platform/`，用 `#[cfg(target_os)]` 隔离，不影响主逻辑。

## 验收标准

- Windows：在 VS Code 光标位于编辑器时，点击/拖动小人后光标仍闪烁，输入不中断。
- macOS：同上，且不切换 Space、不抢菜单栏。
- Linux X11：点击穿透模式下不拦截鼠标；普通模式允许短暂聚焦，但不得把小人窗口置顶到全屏应用之上。
- 三端：右键菜单关闭后焦点回到原应用（或至少不持续占用）。

## 风险

- 平台扩展引入 `unsafe` 代码，需要 `// SAFETY:` 注释与最小化封装。
- macOS 非激活面板可能影响右键菜单/输入框；需要单独验证。
- Windows `WS_EX_NOACTIVATE` 与 Tauri 的窗口拖拽/菜单可能冲突。

## 下一步

- 在 Windows/macOS 实机 上跑最小 Tauri 窗口 spike；
- 记录结果到本文，再决定是否进入正式实现；
- 若 spike 失败，保留 `focus: false` + 点击穿透作为产品级降级方案。
