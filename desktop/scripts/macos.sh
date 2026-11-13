# shellcheck shell=bash
# macOS 桌面入口（由根目录 install-desktop.sh source）。

find_macos_app() {
  local dir
  for dir in \
    "$ROOT/desktop/resources" \
    "$ROOT/backend/target/release/bundle/macos" \
    "$ROOT/desktop/release"; do
    [[ -d "$dir" ]] || continue
    local app
    app="$(find "$dir" -type d -name '*.app' -print 2>/dev/null | head -n 1 || true)"
    if [[ -n "$app" ]]; then
      printf '%s' "$app"
      return 0
    fi
  done
  return 1
}

install_macos() {
  local target="$DESKTOP_DIR/Yuzu.app"
  local command_file="$ROOT/desktop/open-app-macos.command"
  local app=""

  mkdir -p "$DESKTOP_DIR"
  app="$(find_macos_app || true)"

  if [[ -n "$app" ]]; then
    if [[ -e "$target" && ! -L "$target" ]]; then
      echo "桌面已有同名目录，请先移走: $target" >&2
      exit 1
    fi
    rm -f "$target"
    ln -s "$app" "$target"
    echo "已创建 macOS 应用快捷方式: $target -> $app"
    return
  fi

  require_file "$command_file"
  chmod +x "$command_file"
  ln -sfn "$command_file" "$DESKTOP_DIR/Yuzu.command"
  echo "未找到 .app，已创建 macOS 启动快捷方式: $DESKTOP_DIR/Yuzu.command"
}

remove_macos() {
  # 只移除本脚本创建的符号链接/命令，不递归删除用户自己的 .app。
  [[ -L "$DESKTOP_DIR/Yuzu.app" ]] && rm -f "$DESKTOP_DIR/Yuzu.app"
  rm -f "$DESKTOP_DIR/Yuzu.command"
  echo "已移除 macOS 快捷方式（应用与数据保留）"
}
