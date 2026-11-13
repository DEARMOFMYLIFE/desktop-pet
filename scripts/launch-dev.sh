#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -f "$HOME/.cargo/env" ]]; then
  # Make rustup visible when launched from a desktop session.
  # shellcheck disable=SC1091
  . "$HOME/.cargo/env"
fi

exec npm run tauri -- dev