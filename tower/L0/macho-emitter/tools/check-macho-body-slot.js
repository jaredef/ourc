#!/usr/bin/env node
const fs = require("fs");

const machoPath = process.argv[2];
const bodyPath = process.argv[3];
if (!machoPath || !bodyPath) {
  process.stderr.write("usage: check-macho-body-slot.js <macho> <body>\n");
  process.exit(1);
}

const macho = fs.readFileSync(machoPath);
const body = fs.readFileSync(bodyPath);
const first = macho.indexOf(body);
if (first < 0) {
  process.stderr.write("body byte span not found in Mach-O image\n");
  process.exit(1);
}
const second = macho.indexOf(body, first + 1);
if (second >= 0) {
  process.stderr.write(`body byte span appears more than once: ${first}, ${second}\n`);
  process.exit(1);
}

const prefix = macho.subarray(0, first);
const suffix = macho.subarray(first + body.length);
const recomposed = Buffer.concat([prefix, body, suffix]);
if (!recomposed.equals(macho)) {
  process.stderr.write("prefix + body + suffix does not recompose Mach-O image\n");
  process.exit(1);
}

process.stdout.write(JSON.stringify({
  bodyOffset: first,
  bodyBytes: body.length,
  prefixBytes: prefix.length,
  suffixBytes: suffix.length,
  machoBytes: macho.length,
}) + "\n");
