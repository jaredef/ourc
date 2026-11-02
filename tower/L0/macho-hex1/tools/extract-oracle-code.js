#!/usr/bin/env node

const fs = require("fs");

const input = fs.readFileSync(0, "utf8");
const words = [];

for (const line of input.split(/\n/)) {
  const parts = line.trim().split(/\s+/);
  if (!/^[0-9a-fA-F]{16}$/.test(parts[0] || "")) continue;
  for (const word of parts.slice(1)) {
    if (/^[0-9a-fA-F]{8}$/.test(word)) words.push(parseInt(word, 16) >>> 0);
  }
}

const out = Buffer.alloc(words.length * 4);
words.forEach((word, index) => out.writeUInt32LE(word, index * 4));
process.stdout.write(out);
