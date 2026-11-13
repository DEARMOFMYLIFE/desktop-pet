#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -n "${XIAOZHU_NATIVE_BINARY:-}" && -x "${XIAOZHU_NATIVE_BINARY}" && -n "${XIAOZHU_NATIVE_LIBDIR:-}" && -d "${XIAOZHU_NATIVE_LIBDIR}" ]]; then
  if command -v bwrap >/dev/null 2>&1; then
    exec bwrap \
      --bind / / \
      --bind "$XIAOZHU_NATIVE_LIBDIR" /usr/lib/x86_64-linux-gnu \
      --dev-bind /dev /dev \
      --proc /proc \
      /usr/bin/env DISPLAY="${DISPLAY:-:0}" GDK_BACKEND=x11 \
      LIBGL_ALWAYS_SOFTWARE=1 \
      "$XIAOZHU_NATIVE_BINARY"
  fi
  export LD_LIBRARY_PATH="$XIAOZHU_NATIVE_LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  exec "$XIAOZHU_NATIVE_BINARY"
fi

if [[ -x "$ROOT/backend/target/release/xiaozhu" ]]; then
  exec "$ROOT/backend/target/release/xiaozhu"
fi

if [[ -x "$ROOT/backend/target/debug/xiaozhu" ]]; then
  exec "$ROOT/backend/target/debug/xiaozhu"
fi

exec "$ROOT/scripts/launch-dev.sh"