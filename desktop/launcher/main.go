// Yuzu desktop launcher.
//
// It locates the project/app relative to the executable instead of relying on
// the current working directory. On Linux it can optionally enter a local
// sysroot with bubblewrap, which is useful on machines without the system
// WebKitGTK 4.1 development/runtime packages.
package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

const appTitle = "小柚 Yuzu"

func main() {
	exe, err := os.Executable()
	if err != nil {
		fail("无法定位启动器", err.Error())
		return
	}
	if resolved, resolveErr := filepath.EvalSymlinks(exe); resolveErr == nil {
		exe = resolved
	}
	root := findRoot(filepath.Dir(exe))
	loadEnvFile(filepath.Join(root, "desktop", "launcher.env"))

	if hasArg("--help") || hasArg("-h") {
		printHelp(root)
		return
	}
	if runtime.GOOS == "linux" && os.Getenv("DISPLAY") == "" && os.Getenv("WAYLAND_DISPLAY") == "" {
		fail("无法打开小柚", "当前没有图形显示（DISPLAY / WAYLAND_DISPLAY 为空）。\n\n浏览器预览：\n  "+filepath.Join(root, "start.sh")+"\n\n启动器日志：\n  "+logPath())
		return
	}

	app, bundle, err := findApp(root)
	if err != nil {
		fail("找不到小柚程序", err.Error()+"\n\n先构建 Tauri 应用：\n  npm run tauri -- build\n\n或准备本地运行时：\n  desktop/launcher/prepare.sh\n\n启动器日志：\n  "+logPath())
		return
	}

	logf("launcher root=%s app=%s bundle=%t", root, app, bundle)
	cmd, err := makeCommand(root, app, bundle, appArgs())
	if err != nil {
		fail("启动失败", err.Error()+"\n\n启动器日志：\n  "+logPath())
		return
	}
	cmd.Dir = root
	if logFile, logErr := openLog(); logErr == nil {
		cmd.Stdout = logFile
		cmd.Stderr = logFile
	}

	if err := cmd.Start(); err != nil {
		fail("启动失败", err.Error()+"\n\n启动器日志：\n  "+logPath())
		return
	}
	logf("started pid=%d", cmd.Process.Pid)

	// Give a failing binary enough time to report an error, then let the app
	// continue independently of this short-lived launcher.
	done := make(chan error, 1)
	go func() { done <- cmd.Wait() }()
	select {
	case err := <-done:
		if err != nil {
			fail("小柚已退出", err.Error()+"\n\n启动器日志：\n  "+logPath())
		}
	case <-time.After(900 * time.Millisecond):
	}
}

func findRoot(start string) string {
	dir := start
	for i := 0; i < 6; i++ {
		if fileExists(filepath.Join(dir, "backend", "tauri.conf.json")) &&
			fileExists(filepath.Join(dir, "package.json")) {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	return filepath.Dir(start)
}

func findApp(root string) (string, bool, error) {
	if override := os.Getenv("YUZU_NATIVE_BINARY"); override != "" {
		path := absoluteFrom(root, override)
		if executable(path) {
			return path, false, nil
		}
		return "", false, fmt.Errorf("YUZU_NATIVE_BINARY 指向的程序不存在或不可执行：%s", path)
	}

	exeName := "yuzu"
	if runtime.GOOS == "windows" {
		exeName += ".exe"
	}
	candidates := []string{
		filepath.Join(root, "desktop", "resources", "bin", exeName),
		filepath.Join(root, "backend", "target", "release", exeName),
		filepath.Join(root, "backend", "target", "debug", exeName),
	}
	for _, candidate := range candidates {
		if executable(candidate) {
			return candidate, false, nil
		}
	}

	if runtime.GOOS == "darwin" {
		for _, dir := range []string{
			filepath.Join(root, "desktop", "resources"),
			filepath.Join(root, "backend", "target", "release", "bundle", "macos"),
			filepath.Join(root, "desktop", "release"),
		} {
			if app := firstBundle(dir); app != "" {
				return app, true, nil
			}
		}
	}
	return "", false, fmt.Errorf("未找到可执行程序。已检查：\n  %s", strings.Join(candidates, "\n  "))
}

func makeCommand(root, app string, bundle bool, args []string) (*exec.Cmd, error) {
	if bundle {
		return exec.Command("open", append([]string{app}, args...)...), nil
	}
	libdir := os.Getenv("YUZU_NATIVE_LIBDIR")
	if libdir != "" {
		libdir = absoluteFrom(root, libdir)
	}
	if runtime.GOOS == "linux" && libdir != "" && os.Getenv("YUZU_USE_BWRAP") != "0" {
		if bwrap, err := exec.LookPath("bwrap"); err == nil {
			target := "/usr/lib/x86_64-linux-gnu"
			if runtime.GOARCH == "arm64" {
				target = "/usr/lib/aarch64-linux-gnu"
			}
			bargs := []string{
				"--bind", "/", "/",
				"--bind", libdir, target,
				"--dev-bind", "/dev", "/dev",
				"--proc", "/proc",
				app,
			}
			cmd := exec.Command(bwrap, append(bargs, args...)...)
			cmd.Env = os.Environ()
			addDisplayBackend(cmd)
			return cmd, nil
		}
	}
	cmd := exec.Command(app, args...)
	cmd.Env = os.Environ()
	if libdir != "" {
		old := os.Getenv("LD_LIBRARY_PATH")
		value := libdir
		if old != "" {
			value += string(os.PathListSeparator) + old
		}
		cmd.Env = append(cmd.Env, "LD_LIBRARY_PATH="+value)
	}
	addDisplayBackend(cmd)
	return cmd, nil
}

func addDisplayBackend(cmd *exec.Cmd) {
	if runtime.GOOS != "linux" {
		return
	}
	if os.Getenv("GDK_BACKEND") == "" && os.Getenv("DISPLAY") != "" && os.Getenv("WAYLAND_DISPLAY") == "" {
		cmd.Env = append(cmd.Env, "GDK_BACKEND=x11")
	}
	// WebKitGTK can keep stale frames on transparent windows when its
	// compositing path is enabled; the launcher sets this before app startup.
	if os.Getenv("WEBKIT_DISABLE_COMPOSITING_MODE") == "" {
		cmd.Env = append(cmd.Env, "WEBKIT_DISABLE_COMPOSITING_MODE=1")
	}
}

func appArgs() []string {
	args := make([]string, 0, len(os.Args)-1)
	for _, arg := range os.Args[1:] {
		if arg != "--help" && arg != "-h" {
			args = append(args, arg)
		}
	}
	return args
}
