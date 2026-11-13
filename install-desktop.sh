#!/usr/bin/env bash
# Yuzu 桌面快捷方式安装 / 卸载入口（Linux / macOS / Windows Git Bash）。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$ROOT/desktop/scripts"
ACTION="install"
NATIVE_BINARY=""
NATIVE_LIBDIR=""

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

# shellcheck source=desktop/scripts/common.sh
source "$SCRIPTS_DIR/common.sh"
OS_KIND="$(detect_os)"
DESKTOP_DIR="$(desktop_dir)"
APPS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"

if [[ "$ACTION" == "install" && -n "$NATIVE_BINARY$NATIVE_LIBDIR" ]]; then
  args=()
  [[ -n "$NATIVE_BINARY" ]] && args+=(--binary "$NATIVE_BINARY")
  [[ -n "$NATIVE_LIBDIR" ]] && args+=(--libdir "$NATIVE_LIBDIR")
  "$ROOT/desktop/launcher/prepare.sh" "${args[@]}"
fi

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
