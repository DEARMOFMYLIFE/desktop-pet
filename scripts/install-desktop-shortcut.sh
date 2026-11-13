#!/usr/bin/env bash
# 兼容旧路径；实际安装逻辑已迁移到根目录 install-desktop.sh。
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$ROOT/install-desktop.sh" "$@"
