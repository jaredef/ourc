#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
const title = process.argv[4] || "Structured M0 source for derived Mach-O wrapper";
if (!bodyPath || !identifier) {
  process.stderr.write("usage: write-derived-wrapper-m0.js <body> <identifier> [title]\n");
  process.exit(1);
}

function emitHexSection(name, bytes) {
  process.stdout.write(`\n# segment: ${name}\n`);
  for (let i = 0; i < bytes.length; i += 16) {
    process.stdout.write(bytes.subarray(i, i + 16).toString("hex").match(/../g).join(" "));
    process.stdout.write("\n");
  }
}

function emitM0Source(name, tool) {
  process.stdout.write(`\n# segment constructor: ${name}\n`);
  const source = require("child_process").execFileSync(
    process.execPath,
    [path.join(__dirname, tool), bodyPath, identifier]
  );
  process.stdout.write(source);
  if (source.length > 0 && source[source.length - 1] !== 10) process.stdout.write("\n");
}

process.stdout.write(`# ${title}\n`);
process.stdout.write(`# identifier: ${identifier}\n`);

emitM0Source("derived Mach-O prefix", "write-derived-wrapper-prefix-m0.js");
emitHexSection("arm64 executable body", fs.readFileSync(bodyPath));
emitM0Source("derived link-edit prefix", "write-derived-wrapper-linkedit-prefix-m0.js");
emitM0Source("derived CodeDirectory", "write-derived-wrapper-code-signature-m0.js");
