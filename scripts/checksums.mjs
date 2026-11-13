#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputName = process.argv[2] || 'SHA256SUMS.txt';
const target = join(root, 'backend', 'target');
const bundleExtensions = new Set([
  '.exe',
  '.msi',
  '.dmg',
  '.appimage',
  '.deb',
  '.rpm',
  '.zip',
  '.gz',
  '.sig',
]);

function walk(directory, files = []) {
  let entries;
  try {
    entries = readdirSync(directory, { withFileTypes: true });
  } catch {
    return files;
  }
  for (const entry of entries) {
    const absolute = join(directory, entry.name);
    if (entry.isDirectory()) {
      walk(absolute, files);
    } else if (entry.isFile()) {
      const lower = entry.name.toLowerCase();
      const ext = [...bundleExtensions].find((candidate) => lower.endsWith(candidate));
      if (absolute.includes(`${join('target', '')}`) && absolute.includes('bundle') && ext) {
        files.push(absolute);
      }
    }
  }
  return files;
}

const files = walk(target).sort();
if (files.length === 0) {
  console.error('No bundle artifacts found under backend/target/**/bundle.');
  process.exit(1);
}

const lines = files.map((file) => {
  const hash = createHash('sha256').update(readFileSync(file)).digest('hex');
  return `${hash}  ${relative(root, file)}`;
});

writeFileSync(join(root, outputName), `${lines.join('\n')}\n`);
console.log(`Wrote ${outputName} (${files.length} files)`);
