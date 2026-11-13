#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-release}"
shift || true

NATIVE_BINARY=""
NATIVE_LIBDIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --native-binary)
      NATIVE_BINARY="${2:-}"
      shift 2
      ;;
    --native-libdir)
      NATIVE_LIBDIR="${2:-}"
      shift 2
      ;;
    *)
      echo "unknown option: $1" >&2
      exit 2
      ;;
  esac
done

APPS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
DESKTOP_DIR="$(xdg-user-dir DESKTOP 2>/dev/null || printf '%s/Desktop' "$HOME")"
mkdir -p "$APPS_DIR"

if [[ "$MODE" == "--dev" ]]; then
  EXEC="$ROOT/scripts/launch-desktop.sh"
  ICON="$ROOT/backend/icons/128x128.png"
  if [[ -n "$NATIVE_BINARY" && -n "$NATIVE_LIBDIR" ]]; then
    EXEC="/usr/bin/env XIAOZHU_NATIVE_BINARY=$NATIVE_BINARY XIAOZHU_NATIVE_LIBDIR=$NATIVE_LIBDIR $ROOT/scripts/launch-desktop.sh"
  fi
  NAME="凪"
else
  EXEC="xiaozhu"
  ICON="xiaozhu"
  NAME="凪"
fi

TMP_FILE="$(mktemp)"
trap 'rm -f "$TMP_FILE"' EXIT

cat >"$TMP_FILE" <<EOF
[Desktop Entry]
Type=Application
Version=1.0
Name=$NAME
Name[en]=Nagi
GenericName=Desktop companion
Comment=可爱陪伴 · 番茄钟 · 健康提醒
Comment[en]=Desktop companion, pomodoro and wellness reminders
Exec=$EXEC
Icon=$ICON
Terminal=false
Categories=Utility;
Keywords=pomodoro;focus;timer;companion;凪;番茄钟;
StartupNotify=true
StartupWMClass=Xiaozhu
EOF

install -Dm644 "$TMP_FILE" "$APPS_DIR/com.xiaozhu.app.desktop"

LEGACY_DESKTOP="$DESKTOP_DIR/小驻.desktop"
if [[ -f "$LEGACY_DESKTOP" && ! -f "$DESKTOP_DIR/Nagi.desktop" ]]; then
  mv -f "$LEGACY_DESKTOP" "$DESKTOP_DIR/Nagi.desktop" || true
elif [[ -f "$LEGACY_DESKTOP" ]]; then
  rm -f "$LEGACY_DESKTOP" || true
fi

if [[ -d "$DESKTOP_DIR" ]]; then
  install -m755 "$TMP_FILE" "$DESKTOP_DIR/Nagi.desktop"
  gio set "$DESKTOP_DIR/Nagi.desktop" metadata::trusted true 2>/dev/null || true
fi

update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
desktop-file-validate "$APPS_DIR/com.xiaozhu.app.desktop" || true

echo "installed: $APPS_DIR/com.xiaozhu.app.desktop"
if [[ -d "$DESKTOP_DIR" ]]; then
  echo "desktop:   $DESKTOP_DIR/Nagi.desktop"
fi