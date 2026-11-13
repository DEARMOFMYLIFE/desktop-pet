#!/usr/bin/env bash
# ============================================================================
# 小柚 Yuzu 浏览器预览启动器
#
# 原生应用入口：./desktop/yuzu
# 桌面快捷方式：./install-desktop.sh
#
# 用法：
#   ./start.sh              Vite 开发预览（默认端口 1420）
#   ./start.sh --prod       构建前端后启动生产预览
#   ./start.sh --build      仅构建前端，不启动服务
#   ./start.sh --app        直接启动原生小柚程序
#   ./start.sh --open       服务启动后自动打开设置页
#   ./start.sh --no-open    不自动打开浏览器（默认）
#
# 说明：
#   - 浏览器模式使用 mock IPC，数据与原生 SQLite 隔离；
#   - 脚本自行管理子进程，Ctrl+C 会同时停止服务；
#   - 路径基于脚本位置，可从任意目录执行；
#   - 可用 FRONTEND_PORT 环境变量修改端口。
# ============================================================================
set -euo pipefail

# 项目路径与运行参数默认值。
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${FRONTEND_PORT:-1420}"
MODE="dev"
OPEN_BROWSER=0
SERVER_PID=""

# 输出用法；帮助和参数错误共用同一份文案。
usage() {
  cat <<USAGE
用法: ./start.sh [模式] [选项]

  (默认)          启动 Vite 开发预览
  --prod          构建前端后启动生产预览
  --build         仅构建前端，不启动服务
  --app           直接启动原生小柚程序
  --open          服务启动后自动打开浏览器
  --no-open       不自动打开浏览器（默认）
  -h, --help      显示帮助

浏览器预览使用 mock IPC，数据与原生应用分开。
USAGE
}

# 解析模式与选项；未识别的参数直接退出，避免静默使用错误模式。
while [[ $# -gt 0 ]]; do
  case "$1" in
    --prod) MODE="prod"; shift ;;
    --build) MODE="build"; shift ;;
    --app) MODE="app"; shift ;;
    --open) OPEN_BROWSER=1; shift ;;
    --no-open) OPEN_BROWSER=0; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "未知参数: $1" >&2; usage >&2; exit 2 ;;
  esac
done

# 浏览器模式依赖 Node/npm；原生 --app 模式不需要检查。
need_node() {
  command -v node >/dev/null 2>&1 || { echo "未找到 node（需要 Node 20+）" >&2; exit 1; }
  command -v npm >/dev/null 2>&1 || { echo "未找到 npm" >&2; exit 1; }
}

# 首次运行时安装 workspace 依赖；已安装则直接复用。
ensure_deps() {
  if [[ ! -x "$ROOT/node_modules/.bin/vite" ]]; then
    echo "首次运行，安装前端依赖..."
    (cd "$ROOT" && npm install --no-audit --no-fund)
  fi
}

# 查找 Vite 可执行文件，兼容依赖被 hoist 到根目录的情况。
vite_bin() {
  local bin="$ROOT/node_modules/.bin/vite"
  [[ -x "$bin" ]] || bin="$ROOT/frontend/node_modules/.bin/vite"
  printf '%s' "$bin"
}

# 启动成功后统一打印入口地址，避免用户猜测页面路径。
print_urls() {
  local base="http://127.0.0.1:$PORT"
  echo ""
  echo "════════════════════════════════════════════════════"
  echo "  小柚 Yuzu 浏览器预览已启动"
  echo "  设置:    $base/settings.html"
  echo "  宠物:    $base/pet.html"
  echo "  气泡:    $base/bubble.html"
  echo "  引导:    $base/onboarding.html"
  echo "  提示:    浏览器模式使用 mock IPC，不会读写原生数据"
  echo "  停止:    Ctrl+C"
  echo "════════════════════════════════════════════════════"
}

# 按当前系统调用默认浏览器；失败不影响服务本身。
open_url() {
  local url="$1"
  case "$(uname -s 2>/dev/null || true)" in
    Darwin) command -v open >/dev/null 2>&1 && open "$url" >/dev/null 2>&1 & ;;
    Linux) command -v xdg-open >/dev/null 2>&1 && xdg-open "$url" >/dev/null 2>&1 & ;;
    MINGW*|MSYS*|CYGWIN*) cmd //c start "" "$url" >/dev/null 2>&1 || true ;;
  esac
}

# 退出、Ctrl+C 或子进程结束时清理后台服务。
cleanup() {
  if [[ -n "$SERVER_PID" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

# 原生应用模式：优先使用新 ELF 启动器，其次兼容旧启动脚本。
if [[ "$MODE" == "app" ]]; then
  if [[ -x "$ROOT/desktop/yuzu" ]]; then
    exec "$ROOT/desktop/yuzu" "$@"
  fi
  if [[ -x "$ROOT/scripts/launch-desktop.sh" ]]; then
    exec "$ROOT/scripts/launch-desktop.sh"
  fi
  echo "未找到原生启动入口；请先运行 npm run tauri -- build。" >&2
  exit 1
fi

# 浏览器模式：先确认依赖，再根据模式构建或启动服务。
need_node
ensure_deps

if [[ "$MODE" == "build" ]]; then
  npm run build -w frontend
  echo "前端构建完成: frontend/dist"
  exit 0
fi

# 生产模式使用 vite preview；开发模式直接启动 Vite 开发服务器。
if [[ "$MODE" == "prod" ]]; then
  npm run build -w frontend
  (cd "$ROOT" && npm run preview -w frontend -- --host 127.0.0.1 --port "$PORT" --strictPort) &
else
  if [[ "$PORT" == "1420" ]]; then
    (cd "$ROOT" && npm run dev -w frontend) &
  else
    (cd "$ROOT/frontend" && "$(vite_bin)" --port "$PORT" --strictPort) &
  fi
fi
SERVER_PID=$!

# 留出 Vite 启动时间后打印地址；--open 时打开设置页，脚本继续等待服务。
sleep 1.5
print_urls
if [[ "$OPEN_BROWSER" == "1" ]]; then
  open_url "http://127.0.0.1:$PORT/settings.html"
fi
wait "$SERVER_PID"
