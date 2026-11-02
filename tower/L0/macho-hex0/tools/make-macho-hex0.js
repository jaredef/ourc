#!/usr/bin/env node

const crypto = require("crypto");
const childProcess = require("child_process");
const fs = require("fs");
const path = require("path");

function align(value, alignment) {
  return (value + alignment - 1) & ~(alignment - 1);
}

function u16(buffer, offset, value) {
  buffer.writeUInt16LE(value >>> 0, offset);
}

function u32(buffer, offset, value) {
  buffer.writeUInt32LE(value >>> 0, offset);
}

function u64(buffer, offset, value) {
  buffer.writeBigUInt64LE(BigInt(value), offset);
}

function be32(buffer, offset, value) {
  buffer.writeUInt32BE(value >>> 0, offset);
}

function be64(buffer, offset, value) {
  buffer.writeBigUInt64BE(BigInt(value), offset);
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

function writeHexFile(outPath, bytes, title) {
  const lines = [
    `# ${title}`,
    "# Hand-auditable seed source: hex0 ignores comments and expands the hex",
    "# bytes below into the executable seed.",
    "",
  ];
  for (let i = 0; i < bytes.length; i += 16) {
    lines.push(bytes.subarray(i, i + 16).toString("hex").match(/../g).join(" "));
  }
  lines.push("");
  fs.writeFileSync(outPath, lines.join("\n"));
}

if (process.argv[2] === "--wrap-code") {
  const codePath = process.argv[3];
  const outPath = process.argv[4];
  const identifier = process.argv[5] || "rusteze-wrapped-code";
  if (!codePath || !outPath) {
    process.stderr.write("usage: make-macho-hex0.js --wrap-code <code-bin> <out-macho> [identifier]\n");
    process.exit(1);
  }
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  const code = fs.readFileSync(codePath);
  const uuid = crypto.createHash("md5").update(identifier).digest("hex");
  fs.writeFileSync(outPath, makeMachO(code, { identifier, uuid }), { mode: 0o755 });
  process.exit(0);
}

const outDir = process.argv[2];
if (!outDir) {
  process.stderr.write("usage: make-macho-hex0.js <out-dir>\n");
  process.exit(1);
}
fs.mkdirSync(outDir, { recursive: true });
fs.mkdirSync(path.join(outDir, "fixtures"), { recursive: true });

const hex0Code = wordsToBuffer([
  0xd2a00049, 0xcb2963ff, 0x910003ea, 0xd2800070,
  0xd2800000, 0xaa0a03e1, 0xd2a00022, 0xd4001001,
  0xaa0a03e3, 0x8b000144, 0x91404145, 0xd2800006,
  0xeb04007f, 0x540003c2, 0x38401467, 0x71008cff,
  0x540002a0, 0x7100c0ff, 0x54ffff43, 0x7100e4ff,
  0x540000e9, 0x710184ff, 0x54fffec3, 0x710198ff,
  0x54fffe88, 0x51015ce7, 0x14000002, 0x5100c0e7,
  0xb5000086, 0xaa0703e8, 0xd2800026, 0x17ffffed,
  0xd37ced08, 0xaa070108, 0x380014a8, 0xd2800006,
  0x17ffffe8, 0xeb04007f, 0x540000a2, 0x38401467,
  0x710028ff, 0x54ffff81, 0x17ffffe2, 0xd2800090,
  0xd2800020, 0x91404141, 0xcb0100a2, 0xd4001001,
  0xd2800030, 0xd2800000, 0xd4001001,
]);

const miniExit42Code = Buffer.alloc(12);
u32(miniExit42Code, 0, 0xd2800540);
u32(miniExit42Code, 4, 0xd2800030);
u32(miniExit42Code, 8, 0xd4001001);

const echoOneByteCode = wordsToBuffer([
  0xd10043ff, // sub sp, sp, #0x10
  0x910003e1, // mov x1, sp
  0xd2800070, // mov x16, #3 ; read
  0xd2800000, // mov x0, #0
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0x910003e1, // mov x1, sp
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const echoOneByteLargeStackCode = wordsToBuffer([
  0xd2a00049, // mov x9, #0x20000
  0xcb2963ff, // sub sp, sp, x9
  0x910003ea, // mov x10, sp
  0xd2800070, // mov x16, #3 ; read
  0xd2800000, // mov x0, #0
  0xaa0a03e1, // mov x1, x10
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0xaa0a03e1, // mov x1, x10
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const read64kWriteOneCode = wordsToBuffer([
  0xd2a00049, // mov x9, #0x20000
  0xcb2963ff, // sub sp, sp, x9
  0x910003ea, // mov x10, sp
  0xd2800070, // mov x16, #3 ; read
  0xd2800000, // mov x0, #0
  0xaa0a03e1, // mov x1, x10
  0xd2a00022, // mov x2, #0x10000
  0xd4001001, // svc #0x80
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0xaa0a03e1, // mov x1, x10
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const writeFromOutputWindowCode = wordsToBuffer([
  0xd2a00049, // mov x9, #0x20000
  0xcb2963ff, // sub sp, sp, x9
  0x910003ea, // mov x10, sp
  0x9140414b, // add x11, x10, #0x10, lsl #12
  0x52800b4c, // mov w12, #0x5a
  0x3900016c, // strb w12, [x11]
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0xaa0b03e1, // mov x1, x11
  0xd2800022, // mov x2, #1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const eofWriteZeroCode = wordsToBuffer([
  0xd2a00049, // mov x9, #0x20000
  0xcb2963ff, // sub sp, sp, x9
  0x910003ea, // mov x10, sp
  0xd2800070, // mov x16, #3 ; read
  0xd2800000, // mov x0, #0
  0xaa0a03e1, // mov x1, x10
  0xd2a00022, // mov x2, #0x10000
  0xd4001001, // svc #0x80
  0x91404145, // add x5, x10, #0x10, lsl #12
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0x91404141, // add x1, x10, #0x10, lsl #12
  0xcb0100a2, // sub x2, x5, x1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const onePairHexOnlyCode = wordsToBuffer([
  0xd2a00049, // mov x9, #0x20000
  0xcb2963ff, // sub sp, sp, x9
  0x910003ea, // mov x10, sp
  0xd2800070, // mov x16, #3 ; read
  0xd2800000, // mov x0, #0
  0xaa0a03e1, // mov x1, x10
  0xd2a00022, // mov x2, #0x10000
  0xd4001001, // svc #0x80
  0xaa0a03e3, // mov x3, x10
  0x91404145, // add x5, x10, #0x10, lsl #12
  0x38401467, // ldrb w7, [x3], #1
  0x7100c0ff, // cmp w7, #0x30
  0x5100c0e8, // sub w8, w7, #0x30
  0xd37ced08, // lsl x8, x8, #4
  0x38401467, // ldrb w7, [x3], #1
  0x5100c0e7, // sub w7, w7, #0x30
  0xaa070108, // orr x8, x8, x7
  0x380014a8, // strb w8, [x5], #1
  0xd2800090, // mov x16, #4 ; write
  0xd2800020, // mov x0, #1
  0x91404141, // add x1, x10, #0x10, lsl #12
  0xcb0100a2, // sub x2, x5, x1
  0xd4001001, // svc #0x80
  0xd2800030, // mov x16, #1 ; exit
  0xd2800000, // mov x0, #0
  0xd4001001, // svc #0x80
]);

const onePairNumericCode = wordsToBuffer([
  0xd2a00049, 0xcb2963ff, 0x910003ea, 0xd2800070,
  0xd2800000, 0xaa0a03e1, 0xd2a00022, 0xd4001001,
  0xaa0a03e3, 0x91404145, 0x38401467, 0x5100c0e8,
  0xd37ced08, 0x38401467, 0x5100c0e7, 0xaa070108,
  0x380014a8, 0xd2800090, 0xd2800020, 0x91404141,
  0xcb0100a2, 0xd4001001, 0xd2800030, 0xd2800000,
  0xd4001001,
]);

const onePairLowerAlphaCode = wordsToBuffer([
  0xd2a00049, 0xcb2963ff, 0x910003ea, 0xd2800070,
  0xd2800000, 0xaa0a03e1, 0xd2a00022, 0xd4001001,
  0xaa0a03e3, 0x91404145, 0x38401467, 0x710184ff,
  0x54000063, 0x51015ce7, 0x14000002, 0x5100c0e7,
  0xd37ced08, 0x38401466, 0x710184df, 0x54000063,
  0x51015cc6, 0x14000002, 0x5100c0c6, 0xaa060108,
  0x380014a8, 0xd2800090, 0xd2800020, 0x91404141,
  0xcb0100a2, 0xd4001001, 0xd2800030, 0xd2800000,
  0xd4001001,
]);

const fullLoopNoCommentsCode = wordsToBuffer([
  0xd2a00049, 0xcb2963ff, 0x910003ea, 0xd2800070,
  0xd2800000, 0xaa0a03e1, 0xd2a00022, 0xd4001001,
  0xaa0a03e3, 0x8b000144, 0x91404145, 0xd2800006,
  0xeb04007f, 0x54000302, 0x38401467, 0x7100c0ff,
  0x54ffff83, 0x7100e4ff, 0x540000e9, 0x710184ff,
  0x54ffff03, 0x710198ff, 0x54fffec8, 0x51015ce7,
  0x14000002, 0x5100c0e7, 0xb5000086, 0xaa0703e8,
  0xd2800026, 0x17ffffef, 0xd37ced08, 0xaa070108,
  0x380014a8, 0xd2800006, 0x17ffffea, 0xd2800090,
  0xd2800020, 0x91404141, 0xcb0100a2, 0xd4001001,
  0xd2800030, 0xd2800000, 0xd4001001,
]);

const commentSkipOnlyCode = wordsToBuffer([
  0xd2a00049, 0xcb2963ff, 0x910003ea, 0xd2800070,
  0xd2800000, 0xaa0a03e1, 0xd2a00022, 0xd4001001,
  0xaa0a03e3, 0x8b000144, 0x91404145, 0xeb04007f,
  0x54000162, 0x38401467, 0x71008cff, 0x54000040,
  0x17fffffb, 0xeb04007f, 0x540000c2, 0x38401467,
  0x710028ff, 0x54ffff81, 0x17fffff5, 0xd2800090,
  0xd2800020, 0x91404141, 0xcb0100a2, 0xd4001001,
  0xd2800030, 0xd2800000, 0xd4001001,
]);

function wordsToBuffer(words) {
  const buffer = Buffer.alloc(words.length * 4);
  words.forEach((word, index) => u32(buffer, index * 4, word));
  return buffer;
}

const hex0 = makeMachO(hex0Code, {
  identifier: "rusteze-macho-hex0",
  uuid: "72757374657a652d6d6163686f783000",
  stackSize: 0x100000,
});
const mini = makeMachO(miniExit42Code, {
  identifier: "rusteze-mini-macho",
  uuid: "72757374657a652d6d696e696d6163686f",
});

fs.writeFileSync(path.join(outDir, "hex0.bin"), hex0, { mode: 0o755 });
writeHexFile(path.join(outDir, "hex0.hand.hex"), hex0, "Mach-O arm64 hex0");
writeHexFile(path.join(outDir, "fixtures", "hex0.hex"), hex0, "Mach-O arm64 hex0 raw-byte fixpoint");
writeHexFile(path.join(outDir, "fixtures", "mini-macho.hex"), mini, "Mach-O arm64 exit-42 fixture");

if (process.argv.includes("--probes")) {
  runProbes(outDir);
}

function makeMachO(code, options) {
  const MH_MAGIC_64 = 0xfeedfacf;
  const CPU_TYPE_ARM64 = 0x0100000c;
  const CPU_SUBTYPE_ARM64_ALL = 0;
  const MH_EXECUTE = 2;
  const MH_NOUNDEFS = 0x1;
  const MH_DYLDLINK = 0x4;
  const MH_TWOLEVEL = 0x80;
  const MH_PIE = 0x200000;
  const LC_SEGMENT_64 = 0x19;
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
  const VM_PROT_WRITE = 2;
  const VM_PROT_EXECUTE = 4;
  const S_ATTR_PURE_INSTRUCTIONS = 0x80000000;
  const S_ATTR_SOME_INSTRUCTIONS = 0x00000400;

  const pagezeroCmdSize = 72;
  const textCmdSize = 72 + 80;
  const dataCmdSize = options.dataVmaddr ? 72 : 0;
  const linkeditCmdSize = 72;
  const linkeditDataCmdSize = 16;
  const symtabCmdSize = 24;
  const dysymtabCmdSize = 80;
  const dyldPath = "/usr/lib/dyld";
  const dylibPath = "/usr/lib/libSystem.B.dylib";
  const dyldCmdSize = stringCommandSize(dyldPath, 12);
  const dylibCmdSize = stringCommandSize(dylibPath, 24);
  const uuidCmdSize = 24;
  const buildVersionCmdSize = options.buildTool ? 32 : 24;
  const sourceVersionCmdSize = 16;
  const mainCmdSize = 24;
  const headerSize = 32;
  const ncmds = options.dataVmaddr ? 17 : 16;
  const sizeofcmds = pagezeroCmdSize + textCmdSize + dataCmdSize + linkeditCmdSize +
    linkeditDataCmdSize + linkeditDataCmdSize + symtabCmdSize + dysymtabCmdSize +
    dyldCmdSize + uuidCmdSize + buildVersionCmdSize + sourceVersionCmdSize +
    mainCmdSize + dylibCmdSize + linkeditDataCmdSize + linkeditDataCmdSize +
    linkeditDataCmdSize;

  const textVmaddr = 0x100000000;
  const textFileSize = 0x4000;
  const dataFileOff = options.dataVmaddr ? textFileSize : 0;
  const dataFileSize = options.dataVmaddr ? options.dataVmsize : 0;
  const codeOffset = options.codeOffset || align(headerSize + sizeofcmds, 16);
  if (codeOffset < headerSize + sizeofcmds) {
    throw new Error(`codeOffset ${codeOffset} overlaps load commands ending at ${headerSize + sizeofcmds}`);
  }
  const linkeditOffset = textFileSize + dataFileSize;
  const chainedFixupsOff = linkeditOffset;
  const chainedFixupsSize = 56;
  const exportsTrieOff = chainedFixupsOff + chainedFixupsSize;
  const exportsTrieSize = 48;
  const functionStartsOff = exportsTrieOff + exportsTrieSize;
  const functionStartsSize = 8;
  const symtabOff = functionStartsOff + functionStartsSize;
  const symtabSize = 32;
  const stringTableOff = symtabOff + symtabSize;
  const stringTable = Buffer.alloc(32);
  stringTable[0] = 0;
  stringTable.write("__mh_execute_header", 1, "ascii");
  stringTable.write("_main", 21, "ascii");
  const codeSignatureOff = stringTableOff + stringTable.length;
  const codeDirectoryLength = align(88 + Buffer.byteLength(options.identifier, "utf8") + 1, 4) +
    Math.ceil(codeSignatureOff / 4096) * 32;
  const codeSignatureSize = align(20 + codeDirectoryLength, 16);
  const linkeditSize = align(codeSignatureOff + codeSignatureSize - linkeditOffset, 8);
  const fileSize = linkeditOffset + linkeditSize;
  const linkeditVmaddr = (options.dataVmaddr || (textVmaddr + textFileSize)) +
    (options.dataVmaddr ? options.dataVmsize : 0);
  const mainAddr = textVmaddr + codeOffset;
  const buffer = Buffer.alloc(fileSize);
  let o = 0;

  u32(buffer, o, MH_MAGIC_64); o += 4;
  u32(buffer, o, CPU_TYPE_ARM64); o += 4;
  u32(buffer, o, CPU_SUBTYPE_ARM64_ALL); o += 4;
  u32(buffer, o, MH_EXECUTE); o += 4;
  u32(buffer, o, ncmds); o += 4;
  u32(buffer, o, sizeofcmds); o += 4;
  u32(buffer, o, MH_NOUNDEFS | MH_DYLDLINK | MH_TWOLEVEL | (options.pie === false ? 0 : MH_PIE)); o += 4;
  u32(buffer, o, 0); o += 4;

  writeSegment(buffer, o, "__PAGEZERO", 0, textVmaddr, 0, 0, 0, 0, 0); o += pagezeroCmdSize;
  writeSegment(buffer, o, "__TEXT", textVmaddr, textFileSize, 0, textFileSize,
    VM_PROT_READ | VM_PROT_EXECUTE, VM_PROT_READ | VM_PROT_EXECUTE, 1);
  o += 72;
  fixedAscii(buffer, o, 16, "__text"); o += 16;
  fixedAscii(buffer, o, 16, "__TEXT"); o += 16;
  u64(buffer, o, mainAddr); o += 8;
  u64(buffer, o, code.length); o += 8;
  u32(buffer, o, codeOffset); o += 4;
  u32(buffer, o, 2); o += 4;
  u32(buffer, o, 0); o += 4;
  u32(buffer, o, 0); o += 4;
  u32(buffer, o, S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS); o += 4;
  u32(buffer, o, 0); o += 4;
  u32(buffer, o, 0); o += 4;
  u32(buffer, o, 0); o += 4;
  if (options.dataVmaddr) {
    writeSegment(buffer, o, "__DATA", options.dataVmaddr, options.dataVmsize, dataFileOff, dataFileSize,
      VM_PROT_READ | VM_PROT_WRITE, VM_PROT_READ | VM_PROT_WRITE, 0);
    o += dataCmdSize;
  }
  writeSegment(buffer, o, "__LINKEDIT", linkeditVmaddr, align(linkeditSize, 0x4000),
    linkeditOffset, linkeditSize, VM_PROT_READ, VM_PROT_READ, 0);
  o += linkeditCmdSize;

  writeDataCmd(buffer, o, LC_DYLD_CHAINED_FIXUPS, chainedFixupsOff, chainedFixupsSize); o += 16;
  writeDataCmd(buffer, o, LC_DYLD_EXPORTS_TRIE, exportsTrieOff, exportsTrieSize); o += 16;
  u32(buffer, o, LC_SYMTAB); u32(buffer, o + 4, symtabCmdSize);
  u32(buffer, o + 8, symtabOff); u32(buffer, o + 12, 2);
  u32(buffer, o + 16, stringTableOff); u32(buffer, o + 20, stringTable.length); o += 24;
  u32(buffer, o, LC_DYSYMTAB); u32(buffer, o + 4, dysymtabCmdSize);
  u32(buffer, o + 16, 0); u32(buffer, o + 20, 2); u32(buffer, o + 24, 2); o += 80;
  writeStringCommand(buffer, o, LC_LOAD_DYLINKER, dyldCmdSize, dyldPath, 12); o += dyldCmdSize;
  u32(buffer, o, LC_UUID); u32(buffer, o + 4, uuidCmdSize);
  Buffer.from(options.uuid, "hex").copy(buffer, o + 8); o += 24;
  u32(buffer, o, LC_BUILD_VERSION); u32(buffer, o + 4, buildVersionCmdSize);
  u32(buffer, o + 8, PLATFORM_MACOS);
  u32(buffer, o + 12, options.minos || 0x000e0000);
  u32(buffer, o + 16, options.sdk || 0);
  u32(buffer, o + 20, options.buildTool ? 1 : 0);
  if (options.buildTool) {
    u32(buffer, o + 24, 3);
    u32(buffer, o + 28, options.buildTool);
  }
  o += buildVersionCmdSize;
  u32(buffer, o, LC_SOURCE_VERSION); u32(buffer, o + 4, sourceVersionCmdSize); o += 16;
  u32(buffer, o, LC_MAIN); u32(buffer, o + 4, mainCmdSize);
  u64(buffer, o + 8, codeOffset); u64(buffer, o + 16, options.stackSize || 0); o += 24;
  u32(buffer, o, LC_LOAD_DYLIB); u32(buffer, o + 4, dylibCmdSize); u32(buffer, o + 8, 24);
  u32(buffer, o + 12, 2);
  u32(buffer, o + 16, options.libSystemCurrentVersion || 0);
  u32(buffer, o + 20, options.libSystemCompatibilityVersion || 0);
  buffer.write(dylibPath, o + 24, "utf8"); o += dylibCmdSize;
  writeDataCmd(buffer, o, LC_FUNCTION_STARTS, functionStartsOff, functionStartsSize); o += 16;
  writeDataCmd(buffer, o, LC_DATA_IN_CODE, symtabOff, 0); o += 16;
  writeDataCmd(buffer, o, LC_CODE_SIGNATURE, codeSignatureOff, codeSignatureSize); o += 16;
  if (o !== headerSize + sizeofcmds) throw new Error(`bad command size ${o}`);

  u32(buffer, chainedFixupsOff, 0); u32(buffer, chainedFixupsOff + 4, 32);
  u32(buffer, chainedFixupsOff + 8, 48); u32(buffer, chainedFixupsOff + 12, 48);
  u32(buffer, chainedFixupsOff + 20, 1); u32(buffer, chainedFixupsOff + 32, 3);
  buffer[exportsTrieOff] = 0; buffer[exportsTrieOff + 1] = 1;
  buffer.write("_main", exportsTrieOff + 2, "ascii"); buffer[exportsTrieOff + 8] = 9;
  buffer[exportsTrieOff + 9] = 2; buffer[exportsTrieOff + 11] = codeOffset & 0x7f;
  buffer[exportsTrieOff + 12] = (codeOffset >> 7) & 0x7f;
  buffer[functionStartsOff] = codeOffset & 0x7f;
  buffer[functionStartsOff + 1] = 0x80 | ((codeOffset >> 7) & 0x7f);
  buffer[functionStartsOff + 2] = (codeOffset >> 14) & 0x7f;
  u32(buffer, symtabOff, 1); buffer[symtabOff + 4] = 0x0f; buffer[symtabOff + 5] = 1;
  u64(buffer, symtabOff + 8, textVmaddr);
  u32(buffer, symtabOff + 16, 21); buffer[symtabOff + 20] = 0x0f; buffer[symtabOff + 21] = 1;
  u64(buffer, symtabOff + 24, mainAddr);
  stringTable.copy(buffer, stringTableOff);
  code.copy(buffer, codeOffset);
  writeSignature(buffer, codeSignatureOff, codeSignatureSize, codeSignatureOff, options.identifier, code.length);
  return buffer;
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

function writeSignature(buffer, offset, size, codeLimit, identifier, execLimit) {
  const pageSize = 4096;
  const nCodeSlots = Math.ceil(codeLimit / pageSize);
  const ident = Buffer.from(`${identifier}\0`);
  const identOffset = 88;
  const hashOffset = align(identOffset + ident.length, 4);
  const cdLength = hashOffset + nCodeSlots * 32;
  const cdOffset = offset + 20;
  buffer.fill(0, offset, offset + size);
  be32(buffer, offset, 0xfade0cc0); be32(buffer, offset + 4, 20 + cdLength);
  be32(buffer, offset + 8, 1); be32(buffer, offset + 16, 20);
  be32(buffer, cdOffset, 0xfade0c02); be32(buffer, cdOffset + 4, cdLength);
  be32(buffer, cdOffset + 8, 0x20400); be32(buffer, cdOffset + 12, 0x20002);
  be32(buffer, cdOffset + 16, hashOffset); be32(buffer, cdOffset + 20, identOffset);
  be32(buffer, cdOffset + 28, nCodeSlots); be32(buffer, cdOffset + 32, codeLimit);
  buffer[cdOffset + 36] = 32; buffer[cdOffset + 37] = 2; buffer[cdOffset + 39] = 12;
  be64(buffer, cdOffset + 72, execLimit); be64(buffer, cdOffset + 80, 1);
  ident.copy(buffer, cdOffset + identOffset);
  for (let i = 0; i < nCodeSlots; i += 1) {
    crypto.createHash("sha256")
      .update(buffer.subarray(i * pageSize, Math.min((i + 1) * pageSize, codeLimit)))
      .digest()
      .copy(buffer, cdOffset + hashOffset + i * 32);
  }
}

function runProbes(outDir) {
  const probeDir = path.join(outDir, "build", "probes");
  fs.rmSync(probeDir, { recursive: true, force: true });
  fs.mkdirSync(probeDir, { recursive: true });

  const containerVariants = [
    ["baseline", {}],
    ["clang-build-version", { minos: 0x000f0000, sdk: 0x000f0500, buildTool: 0x00000400 }],
    ["clang-libsystem-versions", {
      libSystemCurrentVersion: 0x05470000,
      libSystemCompatibilityVersion: 0x00010000,
    }],
    ["clang-build-and-libsystem", {
      minos: 0x000f0000,
      sdk: 0x000f0500,
      buildTool: 0x00000400,
      libSystemCurrentVersion: 0x05470000,
      libSystemCompatibilityVersion: 0x00010000,
    }],
    ["clang-code-offset-728", { codeOffset: 728 }],
    ["no-lc-main-stack", { stackSize: 0 }],
  ];

  const lines = [
    "# Mach-O loader probe results",
    "",
    "| probe | body | file | codesign | execution | stdout |",
    "|---|---|---:|---|---|---|",
  ];

  const bodies = [
    ["echo-one-byte", echoOneByteCode, Buffer.from([0x5a])],
    ["echo-one-byte-large-stack", echoOneByteLargeStackCode, Buffer.from([0x5a])],
    ["read64k-write-one", read64kWriteOneCode, Buffer.from([0x5a])],
    ["write-from-output-window", writeFromOutputWindowCode, Buffer.alloc(0)],
    ["eof-write-zero", eofWriteZeroCode, Buffer.alloc(0)],
    ["one-pair-hex-only", onePairHexOnlyCode, Buffer.from("2a")],
    ["one-pair-numeric", onePairNumericCode, Buffer.from("22")],
    ["one-pair-lower-alpha", onePairLowerAlphaCode, Buffer.from("2a")],
    ["full-loop-no-comments", fullLoopNoCommentsCode, Buffer.from("  2a\n33")],
    ["comment-skip-only", commentSkipOnlyCode, Buffer.from("# hello\n")],
  ];

  for (const [variantName, options] of containerVariants) {
    for (const [bodyName, body, input] of bodies) {
      const name = `${bodyName}-${variantName}`;
      const file = path.join(probeDir, `${name}.macho`);
      const image = makeMachO(body, {
        identifier: `rusteze-probe-${name}`,
        uuid: crypto.createHash("md5").update(name).digest("hex"),
        stackSize: 0x100000,
        ...options,
      });
      fs.writeFileSync(file, image, { mode: 0o755 });
      const fileResult = childProcess.spawnSync("file", ["-b", file], { encoding: "utf8" });
      const codesignResult = childProcess.spawnSync("codesign", ["--verify", "--verbose=4", file], {
        encoding: "utf8",
        timeout: 1000,
      });
      const runResult = childProcess.spawnSync(file, {
        input,
        timeout: 1000,
      });
      const codesign = codesignResult.status === 0 ? "ok" : `fail ${codesignResult.status ?? codesignResult.signal}`;
      const execution = runResult.error
        ? runResult.error.code
        : `status ${runResult.status ?? runResult.signal}`;
      const stdout = runResult.stdout ? runResult.stdout.toString("hex") : "";
      lines.push(`| ${variantName} | ${bodyName} | ${escapeCell(fileResult.stdout.trim())} | ${codesign} | ${execution} | ${stdout || "-"} |`);
    }
  }

  const parserLines = [
    "",
    "## Parser-body probes",
    "",
    "The full parser body is emitted for external watchdog probes at",
    "`build/probes/hex0-parser-body.macho`. It is not run by this Node harness",
    "because an earlier hostile execution shape outlived the parent timeout.",
  ];
  fs.writeFileSync(path.join(probeDir, "hex0-parser-body.macho"), makeMachO(hex0Code, {
    identifier: "rusteze-probe-parser-body",
    uuid: crypto.createHash("md5").update("parser-body").digest("hex"),
    stackSize: 0x100000,
  }), { mode: 0o755 });

  lines.push(...parserLines);

  const report = path.join(probeDir, "README.md");
  fs.writeFileSync(report, `${lines.join("\n")}\n`);
  console.log(report);
}

function escapeCell(text) {
  return text.replace(/\|/g, "\\|");
}
