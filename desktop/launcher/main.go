// Yuzu desktop launcher.
//
// It locates the project/app relative to the executable instead of relying on
// the current working directory. On Linux it can optionally enter a local
// sysroot with bubblewrap, which is useful on machines without the system
// WebKitGTK 4.1 development/runtime packages.
package main

import (
	"os"
	"path/filepath"
	"runtime"
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
	launcherRoot = root
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

func appArgs() []string {
	args := make([]string, 0, len(os.Args)-1)
	for _, arg := range os.Args[1:] {
		if arg != "--help" && arg != "-h" {
			args = append(args, arg)
		}
	}
	return args
}
