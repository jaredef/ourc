#!/usr/bin/env node
const crypto = require("crypto");
const fs = require("fs");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: emit-derived-wrapper-prefix.js <body> <identifier>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function u32(buffer, offset, value) {
  buffer.writeUInt32LE(value >>> 0, offset);
}

function u64(buffer, offset, value) {
  buffer.writeBigUInt64LE(BigInt(value), offset);
}

function fixedAscii(buffer, offset, size, text) {
  buffer.fill(0, offset, offset + size);
  buffer.write(text, offset, Math.min(size, text.length), "ascii");
}

function stringCommandSize(text, fixedBytes) {
  return align(fixedBytes + Buffer.byteLength(text, "utf8") + 1, 8);
}

function writeStringCommand(buffer, offset, cmd, cmdsize, text, textOffset) {
  u32(buffer, offset, cmd);
  u32(buffer, offset + 4, cmdsize);
  u32(buffer, offset + 8, textOffset);
  buffer.write(text, offset + textOffset, "utf8");
}

function writeSegment(buffer, offset, name, vmaddr, vmsize, fileoff, filesize, maxprot, initprot, nsects) {
  u32(buffer, offset, 0x19);
  u32(buffer, offset + 4, 72 + nsects * 80);
  fixedAscii(buffer, offset + 8, 16, name);
  u64(buffer, offset + 24, vmaddr);
  u64(buffer, offset + 32, vmsize);
  u64(buffer, offset + 40, fileoff);
  u64(buffer, offset + 48, filesize);
  u32(buffer, offset + 56, maxprot);
  u32(buffer, offset + 60, initprot);
  u32(buffer, offset + 64, nsects);
}

function writeDataCmd(buffer, offset, cmd, dataoff, datasize) {
  u32(buffer, offset, cmd);
  u32(buffer, offset + 4, 16);
  u32(buffer, offset + 8, dataoff);
  u32(buffer, offset + 12, datasize);
}

const MH_MAGIC_64 = 0xfeedfacf;
const CPU_TYPE_ARM64 = 0x0100000c;
const CPU_SUBTYPE_ARM64_ALL = 0;
const MH_EXECUTE = 2;
const MH_NOUNDEFS = 0x1;
const MH_DYLDLINK = 0x4;
const MH_TWOLEVEL = 0x80;
const MH_PIE = 0x200000;
const LC_DYLD_CHAINED_FIXUPS = 0x80000034;
const LC_DYLD_EXPORTS_TRIE = 0x80000033;
const LC_SYMTAB = 0x2;
const LC_DYSYMTAB = 0xb;
const LC_LOAD_DYLINKER = 0xe;
const LC_UUID = 0x1b;
const LC_BUILD_VERSION = 0x32;
const LC_SOURCE_VERSION = 0x2a;
const LC_MAIN = 0x80000028;
const LC_LOAD_DYLIB = 0xc;
const LC_FUNCTION_STARTS = 0x26;
const LC_DATA_IN_CODE = 0x29;
const LC_CODE_SIGNATURE = 0x1d;
const PLATFORM_MACOS = 1;
const VM_PROT_READ = 1;
const VM_PROT_EXECUTE = 4;
const S_ATTR_PURE_INSTRUCTIONS = 0x80000000;
const S_ATTR_SOME_INSTRUCTIONS = 0x00000400;

const pagezeroCmdSize = 72;
const textCmdSize = 72 + 80;
const linkeditCmdSize = 72;
const linkeditDataCmdSize = 16;
const symtabCmdSize = 24;
const dysymtabCmdSize = 80;
const dyldPath = "/usr/lib/dyld";
const dylibPath = "/usr/lib/libSystem.B.dylib";
const dyldCmdSize = stringCommandSize(dyldPath, 12);
const dylibCmdSize = stringCommandSize(dylibPath, 24);
const uuidCmdSize = 24;
const buildVersionCmdSize = 24;
const sourceVersionCmdSize = 16;
const mainCmdSize = 24;
const headerSize = 32;
const ncmds = 16;
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
const linkeditVmaddr = textVmaddr + textFileSize;
const mainAddr = textVmaddr + codeOffset;
const uuid = crypto.createHash("md5").update(identifier).digest("hex");

const buffer = Buffer.alloc(codeOffset);
let o = 0;

u32(buffer, o, MH_MAGIC_64); o += 4;
u32(buffer, o, CPU_TYPE_ARM64); o += 4;
u32(buffer, o, CPU_SUBTYPE_ARM64_ALL); o += 4;
u32(buffer, o, MH_EXECUTE); o += 4;
u32(buffer, o, ncmds); o += 4;
u32(buffer, o, sizeofcmds); o += 4;
u32(buffer, o, MH_NOUNDEFS | MH_DYLDLINK | MH_TWOLEVEL | MH_PIE); o += 4;
u32(buffer, o, 0); o += 4;

writeSegment(buffer, o, "__PAGEZERO", 0, textVmaddr, 0, 0, 0, 0, 0); o += pagezeroCmdSize;
writeSegment(buffer, o, "__TEXT", textVmaddr, textFileSize, 0, textFileSize,
  VM_PROT_READ | VM_PROT_EXECUTE, VM_PROT_READ | VM_PROT_EXECUTE, 1);
o += 72;
fixedAscii(buffer, o, 16, "__text"); o += 16;
fixedAscii(buffer, o, 16, "__TEXT"); o += 16;
u64(buffer, o, mainAddr); o += 8;
u64(buffer, o, body.length); o += 8;
u32(buffer, o, codeOffset); o += 4;
u32(buffer, o, 2); o += 4;
u32(buffer, o, 0); o += 4;
u32(buffer, o, 0); o += 4;
u32(buffer, o, S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS); o += 4;
u32(buffer, o, 0); o += 4;
u32(buffer, o, 0); o += 4;
u32(buffer, o, 0); o += 4;

writeSegment(buffer, o, "__LINKEDIT", linkeditVmaddr, align(linkeditSize, 0x4000),
  linkeditOffset, linkeditSize, VM_PROT_READ, VM_PROT_READ, 0);
o += linkeditCmdSize;

writeDataCmd(buffer, o, LC_DYLD_CHAINED_FIXUPS, chainedFixupsOff, chainedFixupsSize); o += 16;
writeDataCmd(buffer, o, LC_DYLD_EXPORTS_TRIE, exportsTrieOff, exportsTrieSize); o += 16;
u32(buffer, o, LC_SYMTAB); u32(buffer, o + 4, symtabCmdSize);
u32(buffer, o + 8, symtabOff); u32(buffer, o + 12, 2);
u32(buffer, o + 16, stringTableOff); u32(buffer, o + 20, stringTableSize); o += 24;
u32(buffer, o, LC_DYSYMTAB); u32(buffer, o + 4, dysymtabCmdSize);
u32(buffer, o + 16, 0); u32(buffer, o + 20, 2); u32(buffer, o + 24, 2); o += 80;
writeStringCommand(buffer, o, LC_LOAD_DYLINKER, dyldCmdSize, dyldPath, 12); o += dyldCmdSize;
u32(buffer, o, LC_UUID); u32(buffer, o + 4, uuidCmdSize);
Buffer.from(uuid, "hex").copy(buffer, o + 8); o += 24;
u32(buffer, o, LC_BUILD_VERSION); u32(buffer, o + 4, buildVersionCmdSize);
u32(buffer, o + 8, PLATFORM_MACOS);
u32(buffer, o + 12, 0x000e0000);
u32(buffer, o + 16, 0);
u32(buffer, o + 20, 0);
o += buildVersionCmdSize;
u32(buffer, o, LC_SOURCE_VERSION); u32(buffer, o + 4, sourceVersionCmdSize); o += 16;
u32(buffer, o, LC_MAIN); u32(buffer, o + 4, mainCmdSize);
u64(buffer, o + 8, codeOffset); u64(buffer, o + 16, 0); o += 24;
u32(buffer, o, LC_LOAD_DYLIB); u32(buffer, o + 4, dylibCmdSize); u32(buffer, o + 8, 24);
u32(buffer, o + 12, 2);
u32(buffer, o + 16, 0);
u32(buffer, o + 20, 0);
buffer.write(dylibPath, o + 24, "utf8"); o += dylibCmdSize;
writeDataCmd(buffer, o, LC_FUNCTION_STARTS, functionStartsOff, functionStartsSize); o += 16;
writeDataCmd(buffer, o, LC_DATA_IN_CODE, symtabOff, 0); o += 16;
writeDataCmd(buffer, o, LC_CODE_SIGNATURE, codeSignatureOff, codeSignatureSize); o += 16;

if (o !== headerSize + sizeofcmds) {
  process.stderr.write(`bad derived prefix command size ${o}\n`);
  process.exit(1);
}

process.stdout.write(buffer);
