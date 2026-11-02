#!/usr/bin/env node
const childProcess = require("child_process");
const fs = require("fs");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: write-derived-wrapper-linkedit-prefix-m0.js <body> <identifier>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);
const bytes = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-linkedit-prefix.js"), bodyPath, identifier]
);

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function stringCommandSize(text, fixedBytes) {
  return align(fixedBytes + Buffer.byteLength(text, "utf8") + 1, 8);
}

const headerSize = 32;
const sizeofcmds = 72 + (72 + 80) + 72 + 16 + 16 + 24 + 80 +
  stringCommandSize("/usr/lib/dyld", 12) + 24 + 24 + 16 + 24 +
  stringCommandSize("/usr/lib/libSystem.B.dylib", 24) + 16 + 16 + 16;
const codeOffset = align(headerSize + sizeofcmds, 16);
const linkeditOffset = 0x4000;
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
const start = codeOffset + body.length;

function emitBytes(name, chunk) {
  process.stdout.write(`%${name} ${chunk.toString("hex")}\n`);
  process.stdout.write(`${name}\n`);
}

function emitChunked(name, chunk) {
  process.stdout.write(`\n# link-edit region: ${name}\n`);
  for (let i = 0; i < chunk.length; i += 16) {
    emitBytes(`${name}_${String(i / 16).padStart(4, "0")}`, chunk.subarray(i, i + 16));
  }
}

function emitRawChunked(name, chunk) {
  process.stdout.write(`\n# link-edit region: ${name}\n`);
  for (let i = 0; i < chunk.length; i += 16) {
    process.stdout.write(chunk.subarray(i, i + 16).toString("hex").match(/../g).join(" "));
    process.stdout.write("\n");
  }
}

function slice(absStart, absEnd) {
  return bytes.subarray(absStart - start, absEnd - start);
}

process.stdout.write("# M0 constructor source for derived Mach-O link-edit prefix\n");
process.stdout.write(`# identifier: ${identifier}\n`);

emitRawChunked("post_body_zero_padding", bytes.subarray(0, linkeditOffset - start));
emitChunked("chained_fixups_payload", slice(chainedFixupsOff, chainedFixupsOff + chainedFixupsSize));
emitChunked("exports_trie_payload", slice(exportsTrieOff, exportsTrieOff + exportsTrieSize));
emitChunked("function_starts_payload", slice(functionStartsOff, functionStartsOff + functionStartsSize));
emitChunked("symtab_payload", slice(symtabOff, symtabOff + symtabSize));
emitChunked("string_table_payload", slice(stringTableOff, stringTableOff + stringTableSize));
