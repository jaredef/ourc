#!/usr/bin/env node

const fs = require("fs");

const inputPath = process.argv[2];
const title = process.argv[3] || "arm64 code";
if (!inputPath) {
  process.stderr.write("usage: write-labeled-hex.js <input-bin> [title]\n");
  process.exit(1);
}

const bytes = fs.readFileSync(inputPath);
const lines = [
  `# ${title}`,
  "# Generated hex source.",
  "",
];

for (let i = 0; i < bytes.length; i += 16) {
  lines.push(bytes.subarray(i, i + 16).toString("hex").match(/../g).join(" "));
}

lines.push("");
process.stdout.write(lines.join("\n"));
