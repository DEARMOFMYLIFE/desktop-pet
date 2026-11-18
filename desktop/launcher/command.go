package main

import (
	"os"
	"os/exec"
	"runtime"
)

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
	if os.Getenv("WEBKIT_DISABLE_COMPOSITING_MODE") == "" {
		cmd.Env = append(cmd.Env, "WEBKIT_DISABLE_COMPOSITING_MODE=1")
	}
}
