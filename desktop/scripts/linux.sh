# shellcheck shell=bash
# Linux 桌面入口（由根目录 install-desktop.sh source）。

linux_desktop_file() {
  local target="$1"
  cat >"$target" <<DESKTOP
[Desktop Entry]
Type=Application
Version=1.0
Name=小柚
Name[en]=Yuzu
GenericName=Desktop companion
Comment=可爱陪伴 · 番茄钟 · 健康提醒
Comment[en]=Desktop companion, pomodoro and wellness reminders
Exec="$LAUNCHER"
Path=$ROOT
Icon=$ICON
Terminal=false
Categories=Utility;
Keywords=pomodoro;focus;timer;companion;小柚;番茄钟;
StartupNotify=true
StartupWMClass=Yuzu
DESKTOP
}

install_linux() {
  local apps_dir="$APPS_DIR"
  local desktop_dir="$DESKTOP_DIR"
  LAUNCHER="$ROOT/desktop/yuzu"
  ICON="$HOME/.local/share/yuzu/app-icon.png"
  [[ -f "$ICON" ]] || ICON="$ROOT/backend/icons/128x128.png"

  require_exec "$LAUNCHER"
  require_file "$ICON"
  mkdir -p "$apps_dir" "$desktop_dir"

  linux_desktop_file "$apps_dir/com.yuzu.app.desktop"
  linux_desktop_file "$desktop_dir/Yuzu.desktop"
  chmod 755 "$apps_dir/com.yuzu.app.desktop" "$desktop_dir/Yuzu.desktop"

  # Remove old names from earlier iterations, but never touch user data.
  rm -f "$apps_dir/com.xiaozhu.app.desktop" \
    "$desktop_dir/小驻.desktop" "$desktop_dir/Nagi.desktop" 2>/dev/null || true

  gio set "$desktop_dir/Yuzu.desktop" metadata::trusted true 2>/dev/null || true
  command -v update-desktop-database >/dev/null 2>&1 &&
    update-desktop-database "$apps_dir" >/dev/null 2>&1 || true
  if command -v desktop-file-validate >/dev/null 2>&1; then
    desktop-file-validate "$apps_dir/com.yuzu.app.desktop" || true
  fi

  echo "已安装应用菜单: $apps_dir/com.yuzu.app.desktop"
  echo "已创建桌面快捷方式: $desktop_dir/Yuzu.desktop"
  echo "启动入口: $LAUNCHER"
}

remove_linux() {
  rm -f "$APPS_DIR/com.yuzu.app.desktop" "$DESKTOP_DIR/Yuzu.desktop"
  command -v update-desktop-database >/dev/null 2>&1 &&
    update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
  echo "已移除 Linux 桌面快捷方式（应用与数据保留）"
}
