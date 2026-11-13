$ErrorActionPreference = "Stop"

Set-Location (Split-Path -Parent $PSScriptRoot)

Write-Host "==> frontend checks"
npm run lint
npm test
npm run build

Write-Host "==> desktop bundle"
npm run tauri -- build

Write-Host "==> checksums"
node scripts/checksums.mjs

Write-Host ""
Write-Host "Artifacts:"
Get-ChildItem -Path backend/target -Recurse -File |
    Where-Object { $_.FullName -like "*release*bundle*" } |
    ForEach-Object { $_.FullName }
Write-Host "Checksums: SHA256SUMS.txt"
