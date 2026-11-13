package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

func fail(title, message string) {
	logf("%s: %s", title, message)
	fmt.Fprintln(os.Stderr, title+": "+message)
	if runtime.GOOS == "linux" {
		if zenity, err := exec.LookPath("zenity"); err == nil {
			_ = exec.Command(zenity, "--error", "--title", appTitle, "--text", title+"\n\n"+message, "--width=520").Run()
		} else if notify, err := exec.LookPath("notify-send"); err == nil {
			_ = exec.Command(notify, "-a", appTitle, title, message).Run()
		}
		os.Exit(1)
	}
	if runtime.GOOS == "darwin" {
		script := fmt.Sprintf("display alert %q message %q as critical", appTitle, title+"\n\n"+message)
		_ = exec.Command("osascript", "-e", script).Run()
	}
	os.Exit(1)
}

func printHelp(root string) {
	fmt.Printf("%s 启动器\n\n", appTitle)
	fmt.Printf("用法: %s [应用参数]\n\n", filepath.Join(root, "desktop", "yuzu"))
	fmt.Println("环境变量:")
	fmt.Println("  YUZU_NATIVE_BINARY  指定 Tauri 可执行文件")
	fmt.Println("  YUZU_NATIVE_LIBDIR  本地运行时库目录")
	fmt.Println("  YUZU_USE_BWRAP=0    禁用 bubblewrap")
	fmt.Println("")
	fmt.Printf("配置文件: %s\n", filepath.Join(root, "desktop", "launcher.env"))
}
