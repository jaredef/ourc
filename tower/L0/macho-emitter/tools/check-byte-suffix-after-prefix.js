#!/usr/bin/env node
const fs = require("fs");

const prefixPath = process.argv[2];
const suffixPath = process.argv[3];
const filePath = process.argv[4];
if (!prefixPath || !suffixPath || !filePath) {
  process.stderr.write("usage: check-byte-suffix-after-prefix.js <prefix> <expected-suffix> <file>\n");
  process.exit(1);
}

const prefix = fs.readFileSync(prefixPath);
const expected = fs.readFileSync(suffixPath);
const file = fs.readFileSync(filePath);
if (prefix.length + expected.length > file.length) {
  process.stderr.write("prefix + expected suffix exceeds file length\n");
  process.exit(1);
}

const actual = file.subarray(prefix.length, prefix.length + expected.length);
if (!actual.equals(expected)) {
  for (let i = 0; i < expected.length; i += 1) {
    if (actual[i] !== expected[i]) {
      process.stderr.write(`suffix after prefix differs at ${i}: got ${actual[i]}, expected ${expected[i]}\n`);
      process.exit(1);
    }
  }
}
