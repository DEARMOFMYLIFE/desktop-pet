#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -n "${YUZU_NATIVE_BINARY:-}" && -x "${YUZU_NATIVE_BINARY}" && -n "${YUZU_NATIVE_LIBDIR:-}" && -d "${YUZU_NATIVE_LIBDIR}" ]]; then
  if command -v bwrap >/dev/null 2>&1; then
    exec bwrap \
      --bind / / \
      --bind "$YUZU_NATIVE_LIBDIR" /usr/lib/x86_64-linux-gnu \
      --dev-bind /dev /dev \
      --proc /proc \
      /usr/bin/env DISPLAY="${DISPLAY:-:0}" GDK_BACKEND=x11 \
      LIBGL_ALWAYS_SOFTWARE=1 \
      "$YUZU_NATIVE_BINARY"
  fi
  export LD_LIBRARY_PATH="$YUZU_NATIVE_LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  exec "$YUZU_NATIVE_BINARY"
fi

if [[ -x "$ROOT/backend/target/release/yuzu" ]]; then
  exec "$ROOT/backend/target/release/yuzu"
fi

if [[ -x "$ROOT/backend/target/debug/yuzu" ]]; then
  exec "$ROOT/backend/target/debug/yuzu"
fi

echo "未找到已构建的小柚程序。请先运行: npm run tauri -- build" >&2
if command -v zenity >/dev/null 2>&1; then
  zenity --error --title="小柚" --text="未找到已构建的小柚程序。请先运行: npm run tauri -- build" --width=420 2>/dev/null || true
elif command -v notify-send >/dev/null 2>&1; then
  notify-send "小柚" "未找到已构建的小柚程序，请先运行 npm run tauri -- build" 2>/dev/null || true
fi
exit 1