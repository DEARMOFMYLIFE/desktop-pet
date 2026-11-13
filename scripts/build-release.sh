#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> frontend checks"
npm run lint
npm test
npm run build

echo "==> desktop bundle"
npm run tauri -- build

echo "==> checksums"
node scripts/checksums.mjs

echo
echo "Artifacts:"
find backend/target -path '*release/bundle*' -type f | sort
echo
echo "Checksums: SHA256SUMS.txt"
