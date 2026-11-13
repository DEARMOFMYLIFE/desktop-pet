#!/usr/bin/env bash
# 把已构建的 Tauri 程序放入稳定的 desktop/resources/bin，并可选写入本地
# 运行时配置。Tauri 二进制本身有 20MB+，不提交到仓库。
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BINARY=""
LIBDIR=""

usage() {
  cat <<USAGE
用法: ./desktop/launcher/prepare.sh [--binary PATH] [--libdir PATH]

  --binary   Tauri release/debug 可执行文件；省略时自动查找
  --libdir   本地 WebKitGTK 运行库目录；写入 desktop/launcher.env

示例:
  ./desktop/launcher/prepare.sh \\
    --binary /tmp/xz-native-target/release/yuzu \\
    --libdir /tmp/xz-merged-lib
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --binary) BINARY="${2:-}"; shift 2 ;;
    --libdir) LIBDIR="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "未知参数: $1" >&2; usage >&2; exit 2 ;;
  esac
done

if [[ -z "$BINARY" ]]; then
  for candidate in \
    "${YUZU_NATIVE_BINARY:-}" \
    "$ROOT/backend/target/release/yuzu" \
    "$ROOT/backend/target/debug/yuzu"; do
    if [[ -n "$candidate" && -x "$candidate" ]]; then
      BINARY="$candidate"
      break
    fi
  done
fi

if [[ -z "$BINARY" || ! -x "$BINARY" ]]; then
  echo "未找到可执行文件。请先运行: npm run tauri -- build，或指定 --binary。" >&2
  exit 1
fi

if [[ -n "$LIBDIR" && ! -d "$LIBDIR" ]]; then
  echo "运行库目录不存在: $LIBDIR" >&2
  exit 1
fi

mkdir -p "$ROOT/desktop/resources/bin"
install -m755 "$BINARY" "$ROOT/desktop/resources/bin/yuzu"
echo "已准备程序: desktop/resources/bin/yuzu"

ENV_FILE="$ROOT/desktop/launcher.env"
if [[ -n "$LIBDIR" ]]; then
  escaped="${LIBDIR//\\/\\\\}"
  escaped="${escaped//\"/\\\"}"
  {
    echo "# 由 desktop/launcher/prepare.sh 生成的本地配置（不要提交）"
    echo "YUZU_NATIVE_LIBDIR=\"$escaped\""
    echo "YUZU_USE_BWRAP=1"
    if [[ -z "${WAYLAND_DISPLAY:-}" && -n "${DISPLAY:-}" ]]; then
      echo "GDK_BACKEND=x11"
    fi
    echo "LIBGL_ALWAYS_SOFTWARE=1"
  } > "$ENV_FILE"
  echo "已写入本地运行时配置: desktop/launcher.env"
fi
