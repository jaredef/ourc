#!/usr/bin/env node
const fs = require("fs");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
const fieldsPath = process.argv[4];
if (!bodyPath || !identifier || !fieldsPath) {
  process.stderr.write("usage: check-derived-wrapper-layout.js <body> <identifier> <wrapper-fields-json>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);
const fields = JSON.parse(fs.readFileSync(fieldsPath, "utf8"));

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function stringCommandSize(text, fixedBytes) {
  return align(fixedBytes + Buffer.byteLength(text, "utf8") + 1, 8);
}

const headerSize = 32;
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
const codeDirectoryLength = align(88 + Buffer.byteLength(identifier, "utf8") + 1, 4) +
  Math.ceil(codeSignatureOff / 4096) * 32;
const codeSignatureSize = align(20 + codeDirectoryLength, 16);
const linkeditSize = align(codeSignatureOff + codeSignatureSize - linkeditOffset, 8);
const fileSize = linkeditOffset + linkeditSize;

const expected = {
  header: {
    ncmds: 16,
    sizeofcmds,
    loadCommandsEnd: headerSize + sizeofcmds,
  },
  body: {
    offset: codeOffset,
    bytes: body.length,
  },
  textSection: {
    addr: textVmaddr + codeOffset,
    size: body.length,
    offset: codeOffset,
  },
  main: {
    entryoff: codeOffset,
    stacksize: 0,
  },
  linkedit: {
    offset: linkeditOffset,
    size: linkeditSize,
    chainedFixupsOff,
    chainedFixupsSize,
    exportsTrieOff,
    exportsTrieSize,
    functionStartsOff,
    functionStartsSize,
    symtabOff,
    symtabSize,
    stringTableOff,
    stringTableSize,
  },
  codeSignature: {
    dataoff: codeSignatureOff,
    datasize: codeSignatureSize,
    codeDirectoryLength,
  },
  machoBytes: fileSize,
};

function assertEqual(path, actual, want) {
  if (actual !== want) {
    process.stderr.write(`${path}: got ${actual}, expected ${want}\n`);
    process.exit(1);
  }
}

assertEqual("machoBytes", fields.machoBytes, expected.machoBytes);
assertEqual("header.ncmds", fields.header.ncmds, expected.header.ncmds);
assertEqual("header.sizeofcmds", fields.header.sizeofcmds, expected.header.sizeofcmds);
assertEqual("header.loadCommandsEnd", fields.header.loadCommandsEnd, expected.header.loadCommandsEnd);
assertEqual("body.offset", fields.body.offset, expected.body.offset);
assertEqual("body.bytes", fields.body.bytes, expected.body.bytes);
assertEqual("textSection.addr", fields.textSection.addr, expected.textSection.addr);
assertEqual("textSection.size", fields.textSection.size, expected.textSection.size);
assertEqual("textSection.offset", fields.textSection.offset, expected.textSection.offset);
assertEqual("main.entryoff", fields.main.entryoff, expected.main.entryoff);
assertEqual("main.stacksize", fields.main.stacksize, expected.main.stacksize);
assertEqual("codeSignature.dataoff", fields.codeSignature.dataoff, expected.codeSignature.dataoff);
assertEqual("codeSignature.datasize", fields.codeSignature.datasize, expected.codeSignature.datasize);

process.stdout.write(`${JSON.stringify(expected, null, 2)}\n`);
