#!/usr/bin/env node
const childProcess = require("child_process");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: emit-derived-wrapper-manifest.js <body> <identifier>\n");
  process.exit(1);
}

const body = fs.readFileSync(bodyPath);

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
const identBytes = Buffer.from(`${identifier}\0`);
const codeDirectoryIdentOffset = 88;
const codeDirectoryHashOffset = align(codeDirectoryIdentOffset + identBytes.length, 4);
const pageSize = 4096;
const nCodeSlots = Math.ceil(codeSignatureOff / pageSize);
const codeDirectoryLength = codeDirectoryHashOffset + nCodeSlots * 32;
const codeSignatureSize = align(20 + codeDirectoryLength, 16);
const linkeditSize = align(codeSignatureOff + codeSignatureSize - linkeditOffset, 8);
const machoBytes = linkeditOffset + linkeditSize;

const prefix = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-prefix.js"), bodyPath, identifier]
);
const linkeditPrefix = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-linkedit-prefix.js"), bodyPath, identifier]
);
const signedBytes = Buffer.concat([prefix, body, linkeditPrefix]);
const pageHashes = [];
for (let i = 0; i < nCodeSlots; i += 1) {
  pageHashes.push(crypto.createHash("sha256")
    .update(signedBytes.subarray(i * pageSize, Math.min((i + 1) * pageSize, codeSignatureOff)))
    .digest("hex"));
}

const manifest = {
  version: 1,
  identifier,
  identifierBytes: identBytes.length,
  uuidHex: crypto.createHash("md5").update(identifier).digest("hex"),
  body: {
    bytes: body.length,
    offset: codeOffset,
    address: textVmaddr + codeOffset,
  },
  layout: {
    headerSize,
    sizeofcmds,
    loadCommandsEnd: headerSize + sizeofcmds,
    machoBytes,
    textVmaddr,
    textFileSize,
    linkeditOffset,
    linkeditSize,
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
    codeSignatureOff,
    codeSignatureSize,
  },
  codeDirectory: {
    pageSize,
    nCodeSlots,
    identOffset: codeDirectoryIdentOffset,
    hashOffset: codeDirectoryHashOffset,
    length: codeDirectoryLength,
    executableSegmentLimit: body.length,
    pageHashes,
  },
  fieldSources: {
    fixed: [
      "mach_header magic/cpu/filetype/flags",
      "segment command names/protections",
      "dyld and libSystem paths",
      "symbol table names",
      "CodeDirectory constants",
    ],
    bodyLengthDerived: [
      "__TEXT,__text.size",
      "CodeDirectory execSegLimit",
      "post-body zero padding length",
    ],
    identifierDerived: [
      "LC_UUID.uuid",
      "CodeDirectory identifier",
      "CodeDirectory hashOffset",
      "CodeDirectory length",
      "LC_CODE_SIGNATURE.datasize",
      "__LINKEDIT.filesize",
    ],
    layoutDerived: [
      "body offset/address",
      "LC_MAIN.entryoff",
      "link-edit payload offsets",
      "LC_CODE_SIGNATURE.dataoff",
      "Mach-O file size",
    ],
    hashDerived: [
      "CodeDirectory page hashes",
    ],
  },
};

process.stdout.write(`${JSON.stringify(manifest, null, 2)}\n`);
