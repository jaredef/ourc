#!/usr/bin/env node

const fs = require("fs");

const bodyPath = process.argv[2];
const machoPath = process.argv[3];
const sourcePath = process.argv[4];

if (!bodyPath || !machoPath || !sourcePath) {
  process.stderr.write("usage: measure-artifact.js <body-bin> <macho-bin> <hand-hex>\n");
  process.exit(2);
}

const bodySize = fs.statSync(bodyPath).size;
const machoSize = fs.statSync(machoPath).size;
const sourceSize = fs.statSync(sourcePath).size;
const containerSize = machoSize - bodySize;

console.log(`# macho-hex2 measurement`);
console.log();
console.log(`body_bytes: ${bodySize}`);
console.log(`macho_bytes: ${machoSize}`);
console.log(`container_bytes: ${containerSize}`);
console.log(`hand_hex_source_bytes: ${sourceSize}`);
console.log();
console.log(`Interpretation: body bytes are the seed algorithm surface; container bytes are Mach-O loader/signature surface.`);

