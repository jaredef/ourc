#!/usr/bin/env node
const fs = require("fs");

const source = fs.readFileSync(0, "utf8");
let inUseBody = false;
for (const [index, raw] of source.split(/\r?\n/).entries()) {
  const line = raw.trim();
  if (!line || line.startsWith("#")) continue;
  if (line.startsWith("%")) {
    if (inUseBody) {
      process.stderr.write(`macro definition after use body at line ${index + 1}\n`);
      process.exit(1);
    }
    continue;
  }
  inUseBody = true;
  if (/^[0-9a-fA-F]+$/.test(line)) {
    process.stderr.write(`raw hex token in compute M0 use body at line ${index + 1}: ${line}\n`);
    process.exit(1);
  }
}

process.stdout.write("compute M0 source uses named instruction vocabulary\n");
