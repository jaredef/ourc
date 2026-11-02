#!/usr/bin/env node
const fs = require("fs");

const machoPath = process.argv[2];
const bodyPath = process.argv[3];
if (!machoPath || !bodyPath) {
  process.stderr.write("usage: inspect-macho-wrapper-fields.js <macho> <body>\n");
  process.exit(1);
}

const macho = fs.readFileSync(machoPath);
const body = fs.readFileSync(bodyPath);

function u32(offset) {
  return macho.readUInt32LE(offset);
}

function u64(offset) {
  return Number(macho.readBigUInt64LE(offset));
}

function zstr(offset, size) {
  const end = macho.indexOf(0, offset);
  const stop = end >= offset && end < offset + size ? end : offset + size;
  return macho.subarray(offset, stop).toString("ascii");
}

function fail(message) {
  process.stderr.write(`${message}\n`);
  process.exit(1);
}

const MH_MAGIC_64 = 0xfeedfacf;
const LC_SEGMENT_64 = 0x19;
const LC_MAIN = 0x80000028;
const LC_CODE_SIGNATURE = 0x1d;

if (u32(0) !== MH_MAGIC_64) fail("not a little-endian Mach-O 64 image");

const ncmds = u32(16);
const sizeofcmds = u32(20);
let offset = 32;
let textSection = null;
let main = null;
let codeSignature = null;
const commands = [];

for (let i = 0; i < ncmds; i += 1) {
  const commandOffset = offset;
  const cmd = u32(offset);
  const cmdsize = u32(offset + 4);
  commands.push({ offset: commandOffset, cmd: `0x${cmd.toString(16)}`, cmdsize });
  if (cmdsize < 8) fail(`load command ${i} has invalid size ${cmdsize}`);
  if (offset + cmdsize > macho.length) fail(`load command ${i} extends beyond file`);

  if (cmd === LC_SEGMENT_64) {
    const segname = zstr(offset + 8, 16);
    const nsects = u32(offset + 64);
    let sectionOffset = offset + 72;
    for (let s = 0; s < nsects; s += 1) {
      const sectname = zstr(sectionOffset, 16);
      const sectionSegname = zstr(sectionOffset + 16, 16);
      if (segname === "__TEXT" && sectionSegname === "__TEXT" && sectname === "__text") {
        textSection = {
          commandOffset,
          sectionOffset,
          addr: u64(sectionOffset + 32),
          size: u64(sectionOffset + 40),
          offset: u32(sectionOffset + 48),
          align: u32(sectionOffset + 52),
          flags: u32(sectionOffset + 64),
        };
      }
      sectionOffset += 80;
    }
  } else if (cmd === LC_MAIN) {
    main = {
      commandOffset,
      entryoff: u64(offset + 8),
      stacksize: u64(offset + 16),
    };
  } else if (cmd === LC_CODE_SIGNATURE) {
    codeSignature = {
      commandOffset,
      dataoff: u32(offset + 8),
      datasize: u32(offset + 12),
    };
  }

  offset += cmdsize;
}

if (offset !== 32 + sizeofcmds) {
  fail(`load commands end at ${offset}, expected ${32 + sizeofcmds}`);
}
if (!textSection) fail("missing __TEXT,__text section");
if (!main) fail("missing LC_MAIN");
if (!codeSignature) fail("missing LC_CODE_SIGNATURE");

const bodyOffset = macho.indexOf(body);
if (bodyOffset < 0) fail("body bytes not found in Mach-O image");
if (macho.indexOf(body, bodyOffset + 1) >= 0) fail("body bytes are not unique in Mach-O image");
if (textSection.offset !== bodyOffset) {
  fail(`__TEXT,__text offset ${textSection.offset} does not match body offset ${bodyOffset}`);
}
if (textSection.size !== body.length) {
  fail(`__TEXT,__text size ${textSection.size} does not match body length ${body.length}`);
}
if (main.entryoff !== bodyOffset) {
  fail(`LC_MAIN entryoff ${main.entryoff} does not match body offset ${bodyOffset}`);
}
if (codeSignature.dataoff <= bodyOffset + body.length) {
  fail("code signature overlaps or precedes executable body");
}
if (codeSignature.dataoff + codeSignature.datasize > macho.length) {
  fail("code signature extends beyond file");
}

const result = {
  machoBytes: macho.length,
  header: { ncmds, sizeofcmds, loadCommandsEnd: offset },
  body: { offset: bodyOffset, bytes: body.length },
  textSection,
  main,
  codeSignature,
  linkEditPrefixBytes: codeSignature.dataoff - (bodyOffset + body.length),
  commands,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
