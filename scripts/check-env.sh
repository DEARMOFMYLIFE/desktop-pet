#!/usr/bin/env bash
set -euo pipefail

# Make a rustup-managed toolchain visible in non-login shells.
if [[ -d "$HOME/.cargo/bin" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

fail=0
check() {
  if command -v "$1" >/dev/null 2>&1; then
    echo "ok:      $1"
  else
    echo "missing: $1"
    fail=1
  fi
}

check node
check npm
check rustc
check cargo
check pkg-config

if [[ "$(uname -s)" == "Linux" ]]; then
  if [[ -f /etc/os-release ]]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    if [[ "${ID:-}" == "ubuntu" && "${VERSION_ID:-}" < "22.04" ]]; then
      echo "note:    Ubuntu ${VERSION_ID} ships WebKitGTK 4.0; Tauri 2 requires WebKitGTK 4.1 (Ubuntu 22.04+)."
    fi
  fi

  for package in webkit2gtk-4.1 gtk+-3.0 ayatana-appindicator3-0.1; do
    if pkg-config --exists "$package"; then
      echo "ok:      $package"
    else
      echo "missing: $package"
      fail=1
    fi
  done
fi

if [[ $fail -ne 0 ]]; then
  cat <<'EOF'

Install the missing dependencies first (see docs/04 §10.1):
  Ubuntu: sudo apt install libwebkit2gtk-4.1-dev build-essential libayatana-appindicator3-dev librsvg2-dev
  Rust:   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
EOF
  exit 1
fi

echo "environment ready"
