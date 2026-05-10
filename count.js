// count-lines.js

import fs from 'fs';
import path from 'path';

const ROOT_DIRS = ['src', 'src-tauri'];

const ALLOWED_EXTENSIONS = new Set(['.vue', '.js', '.ts', '.css', '.rs', '.json']);

const IGNORED_DIRS = new Set(['node_modules', 'target', '.git', 'dist', 'build']);

let totalLines = 0;
let totalFiles = 0;

function countLinesInFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');

  const lines = content.split('\n').length;

  totalLines += lines;
  totalFiles += 1;

  console.log(`${lines.toString().padStart(6)}  ${filePath}`);
}

function walk(dir) {
  if (!fs.existsSync(dir)) {
    return;
  }

  const entries = fs.readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);

    if (entry.isDirectory()) {
      if (IGNORED_DIRS.has(entry.name)) {
        continue;
      }

      walk(fullPath);
      continue;
    }

    const ext = path.extname(entry.name);

    if (ALLOWED_EXTENSIONS.has(ext)) {
      countLinesInFile(fullPath);
    }
  }
}

for (const dir of ROOT_DIRS) {
  walk(path.resolve(dir));
}

console.log('\n========================');
console.log(`Файлов: ${totalFiles}`);
console.log(`Строк кода: ${totalLines}`);
