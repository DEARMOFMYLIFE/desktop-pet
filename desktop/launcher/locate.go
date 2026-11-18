package main

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

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
