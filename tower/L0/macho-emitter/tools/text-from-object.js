#!/usr/bin/env node
const childProcess = require("child_process");
const fs = require("fs");

const objectPath = process.argv[2];
if (!objectPath) {
  process.stderr.write("usage: text-from-object.js <object-file>\n");
  process.exit(1);
}

const out = childProcess.execFileSync("otool", ["-t", objectPath], {
  encoding: "utf8",
});

const bytes = [];
for (const line of out.split(/\r?\n/)) {
  const trimmed = line.trim();
  if (!/^[0-9a-fA-F]{16}\s/.test(trimmed)) continue;
  for (const word of trimmed.slice(17).trim().split(/\s+/)) {
    if (/^[0-9a-fA-F]{8}$/.test(word)) {
      const raw = Buffer.from(word, "hex");
      bytes.push(raw[3], raw[2], raw[1], raw[0]);
    }
  }
}

process.stdout.write(Buffer.from(bytes));
