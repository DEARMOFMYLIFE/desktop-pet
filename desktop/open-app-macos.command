#!/usr/bin/env bash
# macOS 启动入口：优先打开 Tauri 的 .app，其次启动 release 二进制。
set -euo pipefail

SOURCE="${BASH_SOURCE[0]}"
while [[ -L "$SOURCE" ]]; do
  DIR="$(cd -P "$(dirname "$SOURCE")" && pwd)"
  SOURCE="$(readlink "$SOURCE")"
  [[ "$SOURCE" = /* ]] || SOURCE="$DIR/$SOURCE"
done
HERE="$(cd -P "$(dirname "$SOURCE")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"

APP="$(find "$HERE/resources" "$ROOT/backend/target/release/bundle/macos" "$ROOT/desktop/release" \
  -type d -name '*.app' -print 2>/dev/null | head -n 1 || true)"

if [[ -n "$APP" ]]; then
  exec open "$APP" --args "$@"
fi

for BIN in "$HERE/resources/bin/yuzu" "$ROOT/backend/target/release/yuzu" "$ROOT/backend/target/debug/yuzu"; do
  if [[ -x "$BIN" ]]; then
    exec "$BIN" "$@"
  fi
done

MESSAGE="未找到小柚程序。\n\n请先运行：\n  npm run tauri -- build"
printf '%b\n' "$MESSAGE" >&2
if command -v osascript >/dev/null 2>&1; then
  osascript -e "display alert \"小柚 Yuzu\" message \"未找到小柚程序。请先运行 npm run tauri -- build\" as critical" >/dev/null 2>&1 || true
fi
exit 1
