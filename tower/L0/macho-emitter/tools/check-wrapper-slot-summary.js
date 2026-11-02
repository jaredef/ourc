#!/usr/bin/env node
const fs = require("fs");

const records = process.argv.slice(2).map((file) => ({
  file,
  ...JSON.parse(fs.readFileSync(file, "utf8")),
}));

if (records.length === 0) {
  process.stderr.write("usage: check-wrapper-slot-summary.js <slot-json>...\n");
  process.exit(1);
}

const offsets = new Set(records.map((record) => record.bodyOffset));
if (offsets.size !== 1) {
  process.stderr.write(`body offsets differ: ${[...offsets].join(", ")}\n`);
  process.exit(1);
}

for (const record of records) {
  if (record.prefixBytes !== record.bodyOffset) {
    process.stderr.write(`${record.file}: prefixBytes does not equal bodyOffset\n`);
    process.exit(1);
  }
  if (record.prefixBytes + record.bodyBytes + record.suffixBytes !== record.machoBytes) {
    process.stderr.write(`${record.file}: slot lengths do not sum to Mach-O length\n`);
    process.exit(1);
  }
}

process.stdout.write(`wrapper body slot verified: offset ${records[0].bodyOffset}, ${records.length} images\n`);
