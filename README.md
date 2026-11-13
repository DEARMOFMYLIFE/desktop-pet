# 凪 Nagi

> 常驻屏幕边缘的可爱陪伴 + 轻量专注助手。

凪是一个跨平台桌面应用：一个 2D 可爱陪伴住在屏幕边缘，会待机、走动、睡觉、对你的点击做出反应；它同时提供番茄钟、计时器、健康提醒、待办和专注统计。完全离线，无账号、无联网、数据仅存本机。

---

## 当前状态：M5 + 全量代码拆分 + Linux 真机验证完成

**M0 已完成**

- Tauri 2 + Rust 主进程 + React 18 / TypeScript / Vite + Tailwind CSS 多页面脚手架
- 透明置顶宠物窗（`pet.html`）、气泡窗（`bubble.html`）、设置窗（`settings.html`）、首次引导（`onboarding.html`）
- 托盘菜单：显示/隐藏、气泡、点击穿透、设置、引导预览、退出
- 单实例：第二次启动聚焦设置窗
- 系统托盘、窗口位置、点击穿透、尺寸/透明度设置
- SQLite 初始化（完整 v1 schema）与设置持久化
- 日志与统一错误骨架
- 中英双语（共享 locale JSON）
- 占位应用图标 / 托盘图标 / icns / ico
- 前端 `lint / build / test` 通过；Rust `cargo fmt / clippy / check` 通过（M0 阶段仅类型检查；Linux 真机验证见下文）

**M1 已完成**

- 角色序列帧管线：`manifest.json` + DOM 精灵背景渲染 + PNG/WebP 双格式回退
- 自动生成的中性小人占位序列帧（M1 占位，M4 已替换为原创 Q 版角色）
- 宠物状态机：idle / walk / sit / sleep / react / drag，1Hz 自主行为 ticker
- 拖拽：4px 阈值、窗口跟随光标、松手吸附最近屏幕边缘
- 位置持久化：显示器名 + 归一化坐标；支持多屏恢复与工作区夹取
- 边缘走动：沿当前最近边缘移动 80–160px，使用工作区（任务栏/Dock 感知）
- 原生小人右键菜单：摸鱼模式、设置、隐藏、退出
- 点击反应与心情 +1（5 分钟冷却）、SQLite 持久化
- `docs/spikes/no-focus.md`：不抢焦点平台验证方案与降级决策

**M2 已完成**

- Rust 计时引擎：番茄钟 / 倒计时 / 正计时，状态机与 1Hz ticker
- 番茄钟阶段：专注 → 短休 / 长休（每 N 轮）→ 专注；自动开始下一阶段可配置
- 暂停 / 继续 / 跳过 / 重置 / 停止；互斥校验
- 计时快照持久化；重启恢复为暂停，过期会话在启动时结算
- `focus_sessions` 记录：开始 / 完成 / 跳过 / 中断，含 local_date
- 气泡窗接入真实计时：倒计时、阶段、进度、控制按钮、快速预设
- 系统通知：专注完成 / 休息结束 / 倒计时结束；首次引导可请求权限
- 全局快捷键：`CmdOrCtrl + Alt + P` 专注、`X` 穿透、`H` 显隐
- 托盘与小人右键菜单接入计时控制
- 设置 → 计时：番茄钟时长、轮次、自动开始、计时器预设可编辑
- 宠物状态联动：专注 → focus，休息 → break，完成 → celebrate
- `cargo clippy --all-targets -D warnings` 通过；前端 `lint / build / test` 通过

**M3 已完成**

- 应用图标 / 托盘图标改为原创扁平图标主题（时钟/星星/月亮/咖啡），
  由 `scripts/icon-themes.py` 生成；默认时钟，可在设置 → 外观切换
- macOS 使用独立的 template 托盘图标（`icon_as_template`）
- 快捷键改键：点击快捷键后录制组合键，含修饰键校验、冲突检测、失败保留旧值、恢复默认
- 开机自启：`tauri-plugin-autostart`，设置变更即时生效，启动时同步
- 关于页：打开数据文件夹、清除全部数据（二次确认后重启）
- 首次引导通知权限按钮接入真实插件
- 通知权限状态命令；快捷键设置持久化
- 图标许可记录：`LICENSES.md`、`docs/13-角色与图标资源.md`

**M4 已完成**

- 角色素材替换为 pzUH 的开源 Q 版角色 Cute Girl：
  - 16 套配色皮肤：珊瑚（默认红发）/ 蜂蜜 / 奶油 / 薄荷 / 蜜桃 / 星夜 / 蔷薇 / 晴空 / 薰衣草 / 翡翠 / 月光 / 海洋 / 莓果 / 可可 / 雪花 / 阳光
  - 每套 11 个动作：idle / walk / sit / sleep / focus / break / celebrate / drag / react / remind / droop
  - Idle / Walk / Jump / Dead 原生全身动画 + 自定义坐姿 / 看书 / 睡觉 / 低落，200×200 WebP 序列帧
  - 设置 → 外观可切换皮肤；全部皮肤 idle 预加载，切换无残影
- 健康提醒：喝水 / 久坐 / 护眼；间隔、活跃时段、专注时策略、5/10 分钟稍后
- 待办：新增/编辑/删除/完成、截止时间、提醒时间、全部/今天/已完成筛选
- 专注统计：今日 / 7 天 / 30 天，专注分钟、完成轮次、完成率、休息分钟、每日柱状图
- 极简养成：完成专注心情 +3/精力 -5，休息恢复精力；低心情进入 droop
- 提醒触发：系统通知 + 气泡提醒卡片 + 小人招手；待办提醒可一键完成
- 关于页/文档补充 Cute Girl（pzUH）许可记录

**M5 已完成（代码/配置）**

- Tauri bundle 元数据：MIT 许可、Productivity 分类、中英文 NSIS 安装器、macOS 12+ 最低版本
- `scripts/build-release.sh` / `.ps1`：lint → test → build → bundle → SHA256
- `scripts/checksums.mjs`：跨平台生成 `SHA256SUMS.txt`
- `.github/workflows/release.yml`：打 `v*` 标签后构建 Windows / macOS x64+arm64 / Linux 并创建 draft release
- `docs/05-发布与验收.md`：产物清单、签名策略、发布前检查、三端冒烟矩阵、回滚
- `CHANGELOG.md`、`LICENSE`（MIT）、`LICENSES.md`（第三方资产）分离

**代码拆分（本轮）**

- 前端：`types/`、`hooks/`、`lib/ipc/`（含 mock 按域拆分）、`components/settings/` 与页面子组件
- 后端：`types/`、`storage/`、`commands/`、`timer/`、`pet/`、`reminders/`、`windows/` 按域拆分
- 所有前后端源文件均 ≤ 200 行（前端最大 194 行，后端最大 194 行）
- 拆分方案、目录树与映射见 [docs/06-代码拆分与目录结构](docs/06-代码拆分与目录结构.md)

**UI 基础（本轮）**

- 样式迁移到 **Tailwind CSS v3 + PostCSS**；保留 `tokens.css` 设计变量，语义类通过 `@apply` 实现
- 图标统一使用 **Lucide React** 线性图标，不再使用 emoji 图标
- 设置页拆出 `SettingsSidebar` / `SettingsHeader` 布局组件；气泡继续复用 `BubbleShell`
- 新增 `Button` / `IconButton` / `Input` / `Textarea` / `NumberInput` / `Select` / `SearchField` / `Checkbox` / `Card` 项目级组件
- 原生 `<select>` 已替换为可键盘操作的自定义 `Select`
- 统一 `ConfirmDialog` 危险操作确认、`LoadingPane` 加载态、`Toast` 保存反馈
- 数据动作统一经 `lib/ipc/api.ts` 与 hooks，页面组件不直接调用底层 IPC
- TypeScript 保持 `strict`，无 `any`

**Linux 真机验证（2026-11-11）**

- 系统：Ubuntu 22.04.5 LTS，X11 (`DISPLAY=:1`)
- 使用本地 sysroot 解压 Tauri 2 依赖（未修改系统目录），完成真实链接
- `cargo test`：5 个 Rust 单元测试全部通过
- 原生应用启动成功：宠物窗、设置窗、首次引导、气泡/提醒窗均正常显示
- 已完成：首次引导 3 步 → 完成、宠物拖拽/吸附、快捷键启动专注、点击穿透开启/恢复、设置页 8 Tab、自定义下拉、气泡计时、健康提醒气泡
- 新增角色：4 套原创 Q 版皮肤、11 个动作；已修复透明帧叠加残影
- 桌面快捷方式：`~/.local/share/applications/com.xiaozhu.app.desktop` 与 `~/Desktop/Nagi.desktop`，已用 `gtk-launch` 验证可启动
- 唯一环境注意：系统未全局安装 `libwebkit2gtk-4.1-dev` 等开发包，且当前无免密 sudo；长期建议执行 `sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

**待完成**

- Windows / macOS 原生冒烟
- 三端安装包/安装卸载冒烟（M5 验收）
- 更多皮肤 / 角色自定义（P2）

---

## 快速开始

```bash
npm install
npm run dev          # 浏览器预览：http://localhost:1420/pet.html
npm run build        # 类型检查 + 生产构建
npm run lint
npm test
```

浏览器预览使用 mock IPC（设置存 localStorage），可直接打开：

- `http://localhost:1420/pet.html`
- `http://localhost:1420/settings.html`
- `http://localhost:1420/onboarding.html`
- `http://localhost:1420/bubble.html`

### 桌面端

先安装系统依赖（见 [docs/04 §10.1](docs/04-技术方案与工程规范.md)），然后：

```bash
./scripts/check-env.sh
npm run tauri -- dev
npm run tauri -- build
```

### 发布打包

```bash
# macOS / Linux：lint + test + build + bundle + SHA256
./scripts/build-release.sh

# Windows PowerShell
./scripts/build-release.ps1
```

详细产物、签名、冒烟矩阵见 [docs/05-发布与验收](docs/05-发布与验收.md)。

当前开发机（Linux X11）缺 Rust 之外的系统开发包；`scripts/check-env.sh` 会列出缺失项。

---

## 目录

```
desktop-pet/
├── docs/                  # 设计与工程文档 + spikes
├── frontend/              # React 前端
│   ├── public/assets/     # 角色序列帧（4 套皮肤 + manifest）
│   ├── src/
│   │   ├── types/         # 全部共享类型（app/settings/pet/timer/todo/...）
│   │   ├── hooks/         # 业务逻辑（窗口启动、计时、待办、统计、快捷键）
│   │   ├── lib/ipc/       # core / api / defaults / mock 按域拆分
│   │   ├── components/    # ui + settings 公共卡片/行/提示条
│   │   └── windows/       # pet / bubble / settings / onboarding
│   ├── shared/locales/    # 中英共享文案（Rust + 前端）
│   ├── *.html             # 四个窗口入口
│   ├── vite.config.ts
│   └── package.json
├── backend/               # Rust / Tauri 主进程
│   ├── src/
│   │   ├── types/         # 全部 IPC/领域类型
│   │   ├── storage/       # SQLite 按域拆分
│   │   ├── commands/      # Tauri commands 按域拆分
│   │   ├── timer/         # 计时引擎 + 运行时 + 命令
│   │   ├── pet/           # 宠物状态机 + 自主行为
│   │   ├── reminders/     # 健康提醒
│   │   └── windows/       # 窗口/位置/走动
│   ├── capabilities/      # 最小权限
│   ├── icons/             # 应用/托盘图标
│   ├── tauri.conf.json
│   └── Cargo.toml
├── scripts/               # 环境自检 / 图标 / 角色 / 打包 / 桌面快捷方式
├── packaging/             # 正式 Linux .desktop 文件
├── package.json           # npm workspace + Tauri CLI
├── LICENSE                # MIT（项目代码）
├── LICENSES.md            # 第三方资产许可
├── CHANGELOG.md
├── SHA256SUMS-*.txt       # 打包后生成（不入库）
└── README.md
```

---

## 文档

| 文档 | 内容 |
|---|---|
| [01-brainstorming-纪要](docs/01-brainstorming-纪要.md) | 需求访谈、方案对比、范围与风险 |
| [02-PRD](docs/02-PRD.md) | 用户故事、功能需求、流程、数据、验收 |
| [03-交互设计](docs/03-交互设计.md) | 窗口结构、组件拆分、交互流程、动画与文案 |
| [04-技术方案与工程规范](docs/04-技术方案与工程规范.md) | 架构、IPC、SQLite、平台细节、CI、测试、编码规范 |
| [05-发布与验收](docs/05-发布与验收.md) | 产物清单、签名策略、发布前检查、三端冒烟、回滚 |
| [06-代码拆分与目录结构](docs/06-代码拆分与目录结构.md) | 前后端拆分方案、目录树、公共组件/hooks/types 映射 |
| [07-UI 一致性与性能优化](docs/07-UI一致性与性能优化.md) | 样式复用、交互一致性、重渲染/资源/定时器优化 |
| [08-前端设计与组件规范](docs/08-前端设计与组件规范.md) | 视觉方向、tokens、Button/Select/Search/Checkbox/Card 规范 |
| [09-桌面快捷方式](docs/09-桌面快捷方式.md) | 正式/开发桌面项、启动脚本与验证 |
| [10-代码与 UI 一致性复审](docs/10-代码与UI一致性复审.md) | 行数审计、样式复用、间距/交互/文案一致性 |
| [11-页面与组件合规检查](docs/11-页面与组件合规检查.md) | 布局复用、API 分层、严格类型、交互状态矩阵 |
| [12-性能审计与优化](docs/12-性能审计与优化.md) | 重渲染、列表、资源加载、定时器/监听器清理 |
| [13-角色与图标资源](docs/13-角色与图标资源.md) | Cute Girl 角色来源、16 套皮肤、动作映射、图标与许可 |
| [不抢焦点 spike](docs/spikes/no-focus.md) | Windows/macOS/Linux 平台方案与降级决策 |

---

## 验证记录

| 项 | 结果 |
|---|---|
| `npm run lint` | ✅ |
| `npm run build` | ✅（多页面产物：pet/bubble/settings/onboarding） |
| `npm test` | ✅ 6 tests（format + i18n） |
| `cargo fmt --check` | ✅ |
| `cargo clippy --all-targets -- -D warnings` | ✅（本地假 pkg-config） |
| `cargo check --all-targets` | ✅（本地假 pkg-config；未链接） |
| 计时引擎单元测试 | ✅ 5 tests passed（原生链接执行） |
| 角色 sprite 渲染 | ✅ 浏览器截图验证（`.shots/`） |
| 气泡真实计时 UI | ✅ 浏览器截图验证（idle / running） |
| 快捷键改键 UI | ✅ 浏览器截图验证（录制按钮 / 默认值 / 恢复默认） |
| 16 套角色皮肤 | ✅ 浏览器截图验证（外观页 + 宠物渲染） |
| 健康提醒 / 待办 / 统计 UI | ✅ 浏览器截图验证 |
| 应用图标 / 托盘图标 | ✅ 原创图标主题，可在外观切换 |
| 打包配置 / CI release | ✅ 配置完成，未在真实 CI 出包 |
| 代码拆分 | ✅ 所有前后端源文件 ≤ 200 行；方案见 docs/06 |
| Tailwind CSS 迁移 | ✅ 语义类通过 `@apply` 实现，保留 tokens |
| 图标 | ✅ Lucide 线性图标；UI 中已移除 emoji 图标 |
| 一致性与性能 | ✅ ConfirmDialog / LoadingPane / Toast 统一；重渲染与资源加载优化，见 docs/07 |
| 浏览器 UI 截图 | ✅ 设置/宠物/引导/气泡/计时/快捷键/外观/提醒/待办/统计 |
| 原生 `tauri dev` | ⏳ 已确认找到 `backend/`、启动前端、进入 cargo；因 WebKitGTK 4.1 缺失停止 |
| 拖拽/吸附/边缘走动/原生菜单 | ⏳ 代码已通过类型检查，待原生运行验证 |
| 系统通知 / 全局快捷键 / 自启 / 提醒调度 | ⏳ 代码已通过类型检查，待原生运行验证 |
| 三端安装/卸载 | ⏳ 等待 22.04+ 或 GitHub Actions 产物 |

> 本机 Rust 1.99.1、Node 24.14.0、npm 11.9.0、Ubuntu 22.04.5。系统未全局安装 Tauri 2 开发包，本轮通过本地 sysroot 依赖完成真实链接、单元测试与原生运行；正式开发环境建议安装 `libwebkit2gtk-4.1-dev` 等系统包。
>
> 角色：Cute Girl - Free Sprites © pzUH，CC0 1.0（公共领域）；应用图标：项目原创扁平图标主题（MIT）。
> 许可与生成记录见 `LICENSES.md` 与 `docs/13-角色与图标资源.md`。

---

## License

- 项目代码：**MIT**，见 [`LICENSE`](LICENSE)。
- 角色素材：Cute Girl - Free Sprites © pzUH，CC0 1.0（公共领域）。
- 应用图标主题：项目原创扁平图标（MIT）。
- 完整第三方记录见 [`LICENSES.md`](LICENSES.md) 与 `docs/licenses/`。
