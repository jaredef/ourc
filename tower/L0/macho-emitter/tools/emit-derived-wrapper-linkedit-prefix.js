#!/usr/bin/env node
const fs = require("fs");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: emit-derived-wrapper-linkedit-prefix.js <body> <identifier>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function u32At(buffer, base, offset, value) {
  buffer.writeUInt32LE(value >>> 0, offset - base);
}

function u64At(buffer, base, offset, value) {
  buffer.writeBigUInt64LE(BigInt(value), offset - base);
}

function writeAt(buffer, base, offset, text) {
  buffer.write(text, offset - base, "ascii");
}

function stringCommandSize(text, fixedBytes) {
  return align(fixedBytes + Buffer.byteLength(text, "utf8") + 1, 8);
}

const pagezeroCmdSize = 72;
const textCmdSize = 72 + 80;
const linkeditCmdSize = 72;
const linkeditDataCmdSize = 16;
const symtabCmdSize = 24;
const dysymtabCmdSize = 80;
const dyldCmdSize = stringCommandSize("/usr/lib/dyld", 12);
const uuidCmdSize = 24;
const buildVersionCmdSize = 24;
const sourceVersionCmdSize = 16;
const mainCmdSize = 24;
const dylibCmdSize = stringCommandSize("/usr/lib/libSystem.B.dylib", 24);
const headerSize = 32;
const sizeofcmds = pagezeroCmdSize + textCmdSize + linkeditCmdSize +
  linkeditDataCmdSize + linkeditDataCmdSize + symtabCmdSize + dysymtabCmdSize +
  dyldCmdSize + uuidCmdSize + buildVersionCmdSize + sourceVersionCmdSize +
  mainCmdSize + dylibCmdSize + linkeditDataCmdSize + linkeditDataCmdSize +
  linkeditDataCmdSize;

const textVmaddr = 0x100000000;
const textFileSize = 0x4000;
const codeOffset = align(headerSize + sizeofcmds, 16);
const linkeditOffset = textFileSize;
const chainedFixupsOff = linkeditOffset;
const chainedFixupsSize = 56;
const exportsTrieOff = chainedFixupsOff + chainedFixupsSize;
const exportsTrieSize = 48;
const functionStartsOff = exportsTrieOff + exportsTrieSize;
const functionStartsSize = 8;
const symtabOff = functionStartsOff + functionStartsSize;
const symtabSize = 32;
const stringTableOff = symtabOff + symtabSize;
const stringTableSize = 32;
const codeSignatureOff = stringTableOff + stringTableSize;
const mainAddr = textVmaddr + codeOffset;
const start = codeOffset + body.length;

if (start > codeSignatureOff) {
  process.stderr.write(`body end ${start} exceeds link-edit prefix end ${codeSignatureOff}\n`);
  process.exit(1);
}

const buffer = Buffer.alloc(codeSignatureOff - start);

u32At(buffer, start, chainedFixupsOff, 0);
u32At(buffer, start, chainedFixupsOff + 4, 32);
u32At(buffer, start, chainedFixupsOff + 8, 48);
u32At(buffer, start, chainedFixupsOff + 12, 48);
u32At(buffer, start, chainedFixupsOff + 20, 1);
u32At(buffer, start, chainedFixupsOff + 32, 3);

buffer[exportsTrieOff - start] = 0;
buffer[exportsTrieOff - start + 1] = 1;
writeAt(buffer, start, exportsTrieOff + 2, "_main");
buffer[exportsTrieOff - start + 8] = 9;
buffer[exportsTrieOff - start + 9] = 2;
buffer[exportsTrieOff - start + 11] = codeOffset & 0x7f;
buffer[exportsTrieOff - start + 12] = (codeOffset >> 7) & 0x7f;

buffer[functionStartsOff - start] = codeOffset & 0x7f;
buffer[functionStartsOff - start + 1] = 0x80 | ((codeOffset >> 7) & 0x7f);
buffer[functionStartsOff - start + 2] = (codeOffset >> 14) & 0x7f;

u32At(buffer, start, symtabOff, 1);
buffer[symtabOff - start + 4] = 0x0f;
buffer[symtabOff - start + 5] = 1;
u64At(buffer, start, symtabOff + 8, textVmaddr);
u32At(buffer, start, symtabOff + 16, 21);
buffer[symtabOff - start + 20] = 0x0f;
buffer[symtabOff - start + 21] = 1;
u64At(buffer, start, symtabOff + 24, mainAddr);

buffer[stringTableOff - start] = 0;
writeAt(buffer, start, stringTableOff + 1, "__mh_execute_header");
writeAt(buffer, start, stringTableOff + 21, "_main");

if (buffer.length !== codeSignatureOff - (codeOffset + body.length)) {
  process.stderr.write("bad derived link-edit prefix length\n");
  process.exit(1);
}

process.stdout.write(buffer);
