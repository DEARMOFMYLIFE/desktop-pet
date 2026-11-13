@echo off
rem Yuzu Windows 启动入口。安装快捷方式时由 install-desktop.sh 创建 .lnk。
setlocal
cd /d "%~dp0"

if exist "resources\bin\yuzu.exe" (
  start "" "resources\bin\yuzu.exe" %*
  exit /b 0
)
if exist "..\backend\target\release\yuzu.exe" (
  start "" "..\backend\target\release\yuzu.exe" %*
  exit /b 0
)
if exist "..\backend\target\debug\yuzu.exe" (
  start "" "..\backend\target\debug\yuzu.exe" %*
  exit /b 0
)

echo 未找到小柚程序。请先运行: npm run tauri -- build
echo.
pause
exit /b 1
