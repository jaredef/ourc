#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const machoPath = process.argv[2];
const bodyPath = process.argv[3];
const outDir = process.argv[4];
if (!machoPath || !bodyPath || !outDir) {
  process.stderr.write("usage: split-macho-wrapper-template.js <macho> <body> <out-dir>\n");
  process.exit(1);
}

const macho = fs.readFileSync(machoPath);
const body = fs.readFileSync(bodyPath);
const offset = macho.indexOf(body);
if (offset < 0) {
  process.stderr.write("body byte span not found in Mach-O image\n");
  process.exit(1);
}
if (macho.indexOf(body, offset + 1) >= 0) {
  process.stderr.write("body byte span appears more than once\n");
  process.exit(1);
}

const prefix = macho.subarray(0, offset);
const suffix = macho.subarray(offset + body.length);
const recomposed = Buffer.concat([prefix, body, suffix]);
if (!recomposed.equals(macho)) {
  process.stderr.write("prefix + body + suffix does not recompose Mach-O image\n");
  process.exit(1);
}

fs.mkdirSync(outDir, { recursive: true });
fs.writeFileSync(path.join(outDir, "prefix.bin"), prefix);
fs.writeFileSync(path.join(outDir, "body.bin"), body);
fs.writeFileSync(path.join(outDir, "suffix.bin"), suffix);
fs.writeFileSync(path.join(outDir, "recomposed.macho"), recomposed, { mode: 0o755 });
fs.writeFileSync(path.join(outDir, "slot.json"), JSON.stringify({
  bodyOffset: offset,
  bodyBytes: body.length,
  prefixBytes: prefix.length,
  suffixBytes: suffix.length,
  machoBytes: macho.length,
}, null, 2) + "\n");
