#!/usr/bin/env node
const fs = require("fs");

const prefixPath = process.argv[2];
const filePath = process.argv[3];
if (!prefixPath || !filePath) {
  process.stderr.write("usage: check-byte-prefix.js <expected-prefix> <file>\n");
  process.exit(1);
}

const prefix = fs.readFileSync(prefixPath);
const file = fs.readFileSync(filePath);
if (prefix.length > file.length) {
  process.stderr.write(`prefix length ${prefix.length} exceeds file length ${file.length}\n`);
  process.exit(1);
}

const actual = file.subarray(0, prefix.length);
if (!actual.equals(prefix)) {
  for (let i = 0; i < prefix.length; i += 1) {
    if (actual[i] !== prefix[i]) {
      process.stderr.write(`byte prefix differs at ${i}: got ${actual[i]}, expected ${prefix[i]}\n`);
      process.exit(1);
    }
  }
}
