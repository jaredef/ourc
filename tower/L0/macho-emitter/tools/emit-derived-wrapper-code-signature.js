#!/usr/bin/env node
const childProcess = require("child_process");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: emit-derived-wrapper-code-signature.js <body> <identifier>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function be32(buffer, offset, value) {
  buffer.writeUInt32BE(value >>> 0, offset);
}

function be64(buffer, offset, value) {
  buffer.writeBigUInt64BE(BigInt(value), offset);
}

const prefix = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-prefix.js"), bodyPath, identifier]
);
const linkeditPrefix = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-linkedit-prefix.js"), bodyPath, identifier]
);

const codeLimit = prefix.length + body.length + linkeditPrefix.length;
const pageSize = 4096;
const nCodeSlots = Math.ceil(codeLimit / pageSize);
const ident = Buffer.from(`${identifier}\0`);
const identOffset = 88;
const hashOffset = align(identOffset + ident.length, 4);
const cdLength = hashOffset + nCodeSlots * 32;
const size = align(20 + cdLength, 16);
const cdOffset = 20;
const preSignature = Buffer.concat([prefix, body, linkeditPrefix]);
const buffer = Buffer.alloc(size);

be32(buffer, 0, 0xfade0cc0);
be32(buffer, 4, 20 + cdLength);
be32(buffer, 8, 1);
be32(buffer, 16, 20);
be32(buffer, cdOffset, 0xfade0c02);
be32(buffer, cdOffset + 4, cdLength);
be32(buffer, cdOffset + 8, 0x20400);
be32(buffer, cdOffset + 12, 0x20002);
be32(buffer, cdOffset + 16, hashOffset);
be32(buffer, cdOffset + 20, identOffset);
be32(buffer, cdOffset + 28, nCodeSlots);
be32(buffer, cdOffset + 32, codeLimit);
buffer[cdOffset + 36] = 32;
buffer[cdOffset + 37] = 2;
buffer[cdOffset + 39] = 12;
be64(buffer, cdOffset + 72, body.length);
be64(buffer, cdOffset + 80, 1);
ident.copy(buffer, cdOffset + identOffset);

for (let i = 0; i < nCodeSlots; i += 1) {
  crypto.createHash("sha256")
    .update(preSignature.subarray(i * pageSize, Math.min((i + 1) * pageSize, codeLimit)))
    .digest()
    .copy(buffer, cdOffset + hashOffset + i * 32);
}

process.stdout.write(buffer);
