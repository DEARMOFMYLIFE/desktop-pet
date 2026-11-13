#!/usr/bin/env bash
# ============================================================================
# 小柚 Yuzu 桌面快捷方式安装 / 卸载入口
#
# 用法：
#   ./install-desktop.sh                 安装当前平台快捷方式
#   ./install-desktop.sh install         同上
#   ./install-desktop.sh --uninstall     卸载快捷方式（保留应用与数据）
#   ./install-desktop.sh --help          显示帮助
#
# 平台：
#   Linux                desktop/yuzu ELF；应用菜单 + 桌面 .desktop
#   macOS                Tauri .app；缺失时回退 open-app-macos.command
#   Windows Git Bash     PowerShell .lnk；回退 open-app-windows.cmd
#
# 设计：
#   - 根目录脚本只做参数解析与平台分发；
#   - 具体安装/卸载逻辑按平台放在 desktop/scripts/；
#   - 路径基于脚本自身定位，不依赖当前工作目录；
#   - 卸载不会删除应用本体、数据库或用户设置。
# ============================================================================
set -euo pipefail

# 项目根目录与平台脚本目录。
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$ROOT/desktop/scripts"

# 默认值和可选的本地运行时参数。
ACTION="install"
NATIVE_BINARY=""
NATIVE_LIBDIR=""

# 输出用法；帮助和参数错误共用同一份文案。
usage() {
  cat <<USAGE
用法: ./install-desktop.sh [install|uninstall] [选项]

  install, --install     安装当前系统桌面快捷方式（默认）
  uninstall, --uninstall 卸载快捷方式（不删除应用与数据）
  --native-binary PATH   Linux：准备一个本地 Tauri 可执行文件
  --native-libdir PATH   Linux：准备本地运行库目录
  --dev                  兼容旧命令，当前等同于 install
  -h, --help             显示帮助
USAGE
}

# 解析参数；platform 函数只在下方的 case 中加载。
while [[ $# -gt 0 ]]; do
  case "$1" in
    ""|install|--install) ACTION="install"; shift ;;
    uninstall|--uninstall|remove) ACTION="uninstall"; shift ;;
    --native-binary) NATIVE_BINARY="${2:-}"; shift 2 ;;
    --native-libdir) NATIVE_LIBDIR="${2:-}"; shift 2 ;;
    --dev) shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "未知参数: $1" >&2; usage >&2; exit 2 ;;
  esac
done

# 读取系统识别、桌面目录等公共能力。
# shellcheck source=desktop/scripts/common.sh
source "$SCRIPTS_DIR/common.sh"
OS_KIND="$(detect_os)"
DESKTOP_DIR="$(desktop_dir)"
APPS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"

# 可选：先把本机 Tauri 程序和运行库准备好，再写桌面入口。
# 这样双击快捷方式时不需要依赖启动它的终端环境。
if [[ "$ACTION" == "install" && -n "$NATIVE_BINARY$NATIVE_LIBDIR" ]]; then
  args=()
  [[ -n "$NATIVE_BINARY" ]] && args+=(--binary "$NATIVE_BINARY")
  [[ -n "$NATIVE_LIBDIR" ]] && args+=(--libdir "$NATIVE_LIBDIR")
  "$ROOT/desktop/launcher/prepare.sh" "${args[@]}"
fi

# 按平台分发；未知系统明确报错，避免写错其他平台的入口。
case "$OS_KIND" in
  linux)
    # shellcheck source=desktop/scripts/linux.sh
    source "$SCRIPTS_DIR/linux.sh"
    if [[ "$ACTION" == "install" ]]; then install_linux; else remove_linux; fi
    ;;
  macos)
    # shellcheck source=desktop/scripts/macos.sh
    source "$SCRIPTS_DIR/macos.sh"
    if [[ "$ACTION" == "install" ]]; then install_macos; else remove_macos; fi
    ;;
  windows)
    # shellcheck source=desktop/scripts/windows.sh
    source "$SCRIPTS_DIR/windows.sh"
    if [[ "$ACTION" == "install" ]]; then install_windows; else remove_windows; fi
    ;;
  *)
    echo "不支持的操作系统: $(uname -s 2>/dev/null || echo unknown)" >&2
    echo "支持 Linux、macOS 和 Windows Git Bash。" >&2
    exit 1
    ;;
esac
