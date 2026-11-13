# Changelog

## v0.1.0 — 2026-11-09

### Added

- 桌面小人「小柚 Yuzu」：透明置顶宠物窗、屏幕边缘活动、拖拽/吸附、点击反馈。
- 4 套原创 Q 版角色皮肤（女生/男生 × 四种配色），每套 11 个动作序列帧。
- 番茄钟 / 倒计时 / 正计时：暂停、继续、跳过、重置、停止、快照恢复。
- 气泡计时窗、托盘菜单、小人右键菜单。
- 健康提醒：喝水 / 久坐 / 护眼、活跃时段、专注策略、稍后提醒。
- 待办：CRUD、截止/提醒时间、筛选、到点提醒。
- 专注统计：今日 / 7 天 / 30 天、完成率、每日柱状图。
- 极简养成：心情/精力、低心情 droop。
- 中英双语、首次引导、开机自启、全局快捷键、通知权限。
- 快捷键改键与冲突检测。
- 数据目录打开、清除全部数据。
- Twemoji CC-BY 4.0 应用/托盘图标。
- Tailwind CSS v3 + PostCSS 样式体系，保留设计 Token 与语义类。
- Lucide React 线性图标替换界面 emoji 图标。
- 设置页抽出侧边栏 / Header 布局组件。
- 原创 Q 版角色（4 套皮肤、11 动作）与 Canvas 残影修复。
- 自定义下拉、搜索框、Checkbox、数字步进输入、Button/Card 组件。
- Linux 桌面快捷方式与开发启动脚本。

### Packaging

- Tauri 2 bundle targets: Windows NSIS/MSI, macOS DMG (x64/arm64), Linux AppImage/deb.
- GitHub Actions CI + tag-triggered draft release with SHA256 checksums.
- MIT license + third-party asset licenses.

### Known limitations

- v0.1.0 为未签名构建；Windows/macOS 会显示安全提示。
- Linux Wayland 的透明/穿透能力受桌面环境限制。
- 无自动更新。
