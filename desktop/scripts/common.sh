# shellcheck shell=bash
# 平台识别与桌面目录，由 install-desktop.sh source。

detect_os() {
  case "$(uname -s 2>/dev/null || true)" in
    Linux*) echo linux ;;
    Darwin) echo macos ;;
    MINGW*|MSYS*|CYGWIN*) echo windows ;;
    *) echo unknown ;;
  esac
}

desktop_dir() {
  case "$OS_KIND" in
    linux)
      local dir
      dir="$(xdg-user-dir DESKTOP 2>/dev/null || true)"
      [[ -n "$dir" ]] || dir="$HOME/Desktop"
      printf '%s' "$dir"
      ;;
    macos)
      printf '%s/Desktop' "$HOME"
      ;;
    windows)
      local home="${USERPROFILE:-}"
      if [[ -n "$home" ]] && command -v cygpath >/dev/null 2>&1; then
        home="$(cygpath -u "$home")"
      fi
      printf '%s/Desktop' "${home:-$HOME}"
      ;;
    *)
      printf '%s/Desktop' "$HOME"
      ;;
  esac
}

require_file() {
  if [[ ! -f "$1" ]]; then
    echo "未找到文件: $1" >&2
    exit 1
  fi
}

require_exec() {
  if [[ ! -x "$1" ]]; then
    echo "未找到可执行文件: $1" >&2
    exit 1
  fi
}
