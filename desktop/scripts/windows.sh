# shellcheck shell=bash
# Windows Git Bash 桌面入口（由根目录 install-desktop.sh source）。

install_windows() {
  local cmd="$ROOT/desktop/open-app-windows.cmd"
  local lnk="$DESKTOP_DIR/Yuzu.lnk"
  local ps win_cmd win_work win_lnk script

  require_file "$cmd"
  mkdir -p "$DESKTOP_DIR"
  command -v cygpath >/dev/null 2>&1 || {
    echo "当前 Git Bash 缺少 cygpath，无法创建 Windows 快捷方式。" >&2
    exit 1
  }

  win_cmd="$(cygpath -w "$cmd")"
  win_work="$(cygpath -w "$ROOT/desktop")"
  win_lnk="$(cygpath -w "$lnk")"
  ps="$(command -v powershell.exe || command -v pwsh || true)"

  if [[ -n "$ps" ]]; then
    script="$(mktemp "${TMPDIR:-/tmp}/yuzu-shortcut-XXXX.ps1")"
    cat >"$script" <<PS
\$ws = New-Object -ComObject WScript.Shell
\$sc = \$ws.CreateShortcut("$win_lnk")
\$sc.TargetPath = \$env:ComSpec
\$sc.Arguments = '/c ""' + "$win_cmd" + '""'
\$sc.WorkingDirectory = "$win_work"
\$sc.WindowStyle = 1
\$sc.Save()
PS
    if "$ps" -NoProfile -ExecutionPolicy Bypass -File "$(cygpath -w "$script")" >/dev/null 2>&1; then
      rm -f "$script"
      echo "已创建 Windows 桌面快捷方式: $lnk"
      return
    fi
    rm -f "$script"
  fi

  # 无 PowerShell 时生成一个指向绝对路径的 .cmd 快捷方式。
  printf '@echo off\r\ncall "%s" %%*\r\n' "$(cygpath -w "$cmd")" >"$DESKTOP_DIR/Yuzu.cmd"
  echo "已创建 Windows 启动脚本: $DESKTOP_DIR/Yuzu.cmd"
}

remove_windows() {
  rm -f "$DESKTOP_DIR/Yuzu.lnk" "$DESKTOP_DIR/Yuzu.cmd"
  echo "已移除 Windows 快捷方式（应用与数据保留）"
}
