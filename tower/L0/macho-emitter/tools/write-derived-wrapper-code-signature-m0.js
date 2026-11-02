#!/usr/bin/env node
const childProcess = require("child_process");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: write-derived-wrapper-code-signature-m0.js <body> <identifier>\n");
  process.exit(1);
}

const bytes = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-code-signature.js"), bodyPath, identifier]
);

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

const identLength = Buffer.byteLength(identifier, "utf8") + 1;
const identOffset = 20 + 88;
const hashOffset = 20 + align(88 + identLength, 4);

function emitBytes(name, chunk) {
  process.stdout.write(`%${name} ${chunk.toString("hex")}\n`);
  process.stdout.write(`${name}\n`);
}

function emitFields(section, base, fields) {
  process.stdout.write(`\n# CodeDirectory region: ${section}\n`);
  for (const [name, start, end] of fields) {
    emitBytes(`${section}_${name}`, bytes.subarray(base + start, base + end));
  }
}

function emitChunked(section, start, end, width = 16) {
  process.stdout.write(`\n# CodeDirectory region: ${section}\n`);
  const chunk = bytes.subarray(start, end);
  for (let i = 0; i < chunk.length; i += width) {
    emitBytes(`${section}_${String(i / width).padStart(3, "0")}`, chunk.subarray(i, i + width));
  }
}

process.stdout.write("# M0 constructor source for derived Mach-O CodeDirectory\n");
process.stdout.write(`# identifier: ${identifier}\n`);

emitFields("superblob", 0, [
  ["magic", 0, 4],
  ["length", 4, 8],
  ["count", 8, 12],
  ["index_type", 12, 16],
  ["index_offset", 16, 20],
]);

emitFields("codedirectory", 20, [
  ["magic", 0, 4],
  ["length", 4, 8],
  ["version", 8, 12],
  ["flags", 12, 16],
  ["hashOffset", 16, 20],
  ["identOffset", 20, 24],
  ["nSpecialSlots", 24, 28],
  ["nCodeSlots", 28, 32],
  ["codeLimit", 32, 36],
  ["hashSize_hashType_platform_pageSize", 36, 40],
  ["spare2", 40, 44],
  ["scatterOffset", 44, 48],
  ["teamOffset", 48, 52],
  ["spare3", 52, 56],
  ["codeLimit64", 56, 64],
  ["execSegBase", 64, 72],
  ["execSegLimit", 72, 80],
  ["execSegFlags", 80, 88],
]);

emitChunked("identifier", identOffset, hashOffset);
emitChunked("page_hashes", hashOffset, bytes.length);
