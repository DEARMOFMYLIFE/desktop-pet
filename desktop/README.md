# desktop

这里是 Yuzu 的多平台启动入口，结构参考 `oc-session-hub/desktop`：

- `yuzu`：Linux x86_64 静态 ELF 启动器（由 `launcher/` 构建，已提交）
- `launcher/`：Go 启动器源码、构建脚本和本地运行时准备脚本
- `open-app-macos.command`：macOS 启动入口
- `open-app-windows.cmd`：Windows 启动入口
- `scripts/`：跨平台桌面快捷方式安装逻辑
- `resources/bin/`：本地放入的真实 Tauri 程序（不提交）
- `resources/lib/`：可选的本地运行库（不提交）

## 打开方式

```bash
# 直接打开原生应用
./desktop/yuzu

# 安装桌面快捷方式
./install-desktop.sh

# 浏览器预览（Vite mock）
./start.sh
```

如果还没有 Tauri 可执行文件：

```bash
npm run tauri -- build

# 当前机器若使用 /tmp 下的 sysroot，也可以准备一份本地运行时
./desktop/launcher/prepare.sh \
  --binary /tmp/xz-native-target/release/yuzu \
  --libdir /tmp/xz-merged-lib

# 重新构建 Linux 启动器
./desktop/launcher/build.sh
```

`desktop/launcher.env` 与 `desktop/resources` 下的程序/运行库都是本机文件，
不进入 Git。Linux ELF 启动器会优先使用 `desktop/resources/bin/yuzu`，其次使用
Tauri 的 `backend/target/release/yuzu`，最后才使用环境变量指定的程序。

## 说明

Linux ELF 是**启动器**，不是把 Tauri/WebKitGTK 打进一个文件。它负责定位程序、
检查显示环境、设置本地库路径并给出可见错误；真正的应用仍是 Tauri 二进制。
需要跨机器分发时，应使用 Tauri 的 AppImage/deb/dmg/NSIS 等安装包。
