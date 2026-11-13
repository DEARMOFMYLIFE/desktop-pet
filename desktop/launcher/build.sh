#!/usr/bin/env bash
# 构建 Yuzu Linux 启动器。默认 x86_64；传入 arm64 可生成 arm64 版本。
set -euo pipefail
cd "$(dirname "$0")"

ARCH="${1:-amd64}"
OUT="../yuzu"
if [[ "$ARCH" != "amd64" ]]; then
  OUT="../yuzu-$ARCH"
fi

case "$ARCH" in
  amd64|arm64) ;;
  *)
    echo "用法: $0 [amd64|arm64]" >&2
    exit 2
    ;;
esac

command -v go >/dev/null 2>&1 || {
  echo "未找到 go；启动器只需要 Go 构建一次，运行时不依赖 Go。" >&2
  exit 1
}

CGO_ENABLED=0 GOOS=linux GOARCH="$ARCH" \
  go build -trimpath -ldflags "-s -w" -o "$OUT" .

chmod 755 "$OUT"
echo "已生成: $(cd .. && pwd)/$(basename "$OUT")"
file "$OUT" 2>/dev/null || true
