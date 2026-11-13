$ErrorActionPreference = "Stop"

$failed = $false
foreach ($command in @("node", "npm", "rustc", "cargo")) {
    if (Get-Command $command -ErrorAction SilentlyContinue) {
        Write-Host "ok:      $command"
    } else {
        Write-Host "missing: $command"
        $failed = $true
    }
}

$webview = Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F1E7E7E7-8C7F-4B5B-9D0B-6C6B8C6B8C6B}" -ErrorAction SilentlyContinue
if ($null -eq $webview) {
    Write-Host "note:    WebView2 registry key not found; install the WebView2 Runtime"
}

if ($failed) {
    Write-Host ""
    Write-Host "Install the missing dependencies first (see docs/04 §10.1)."
    exit 1
}

Write-Host "environment ready"
