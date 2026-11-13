# Changelog

## Unreleased — 2026-11-13

### Changed

- 项目正式更名为 **小柚 Yuzu**；应用品牌、窗口标题、角色名称与桌面标签同步更新。
- 技术标识全量迁移：Rust crate/二进制 `yuzu`、Tauri `com.yuzu.app`、npm `yuzu` / `@yuzu/frontend`、数据目录与日志目录 `yuzu`。
- 默认角色皮肤更新为 **Cute Girl** 的 16 套配色皮肤与 11 种动作状态。
- 设置页新增 10 套应用图标主题、自定义图标导入与桌面品牌同步。
- 新增项目专用日期时间组件、月份/年份快速选择、SVG 清空按钮和统一 Modal。
- 搜索框、时间输入、设置页布局与按钮样式统一到现有 Token 和组件体系。
- README、文档与界面截图更新为当前小柚品牌和界面。

### Removed

- 移除 GitHub Actions 工作流及相关 CI 构建、发布、校验和脚本。
- 移除旧版发布脚本与本地环境自检脚本，构建统一使用 Tauri/本地命令。

### Added

- 旧数据目录、数据库、桌面入口、快捷方式与角色名称的自动迁移。
- 前端 6 个测试与 Rust 5 个 lib 测试。
- 所有前后端源文件和样式文件保持在 200 行以内。

## v0.1.0 — 2026-11-09

### Added

- 透明置顶宠物窗、屏幕边缘活动、拖拽/吸附与点击反馈。
- 番茄钟 / 倒计时 / 正计时：暂停、继续、跳过、重置、停止、快照恢复。
- 气泡计时窗、托盘菜单、小人右键菜单。
- 健康提醒、待办、专注统计、极简养成。
- 中英双语、首次引导、开机自启、全局快捷键、通知权限。
- SQLite 本地存储、完全离线、无账号、无遥测。
- Tailwind CSS v3 + PostCSS、Lucide React 图标、设置页布局组件与共享 UI。
- Linux 桌面快捷方式与启动脚本。

### Packaging

- Tauri 2 bundle targets: Windows NSIS/MSI, macOS DMG (x64/arm64), Linux AppImage/deb.
- MIT license + third-party asset licenses.

### Known limitations

- v0.1.0 为未签名构建；Windows/macOS 会显示安全提示。
- Linux Wayland 的透明/穿透能力受桌面环境限制。
- 无自动更新。
