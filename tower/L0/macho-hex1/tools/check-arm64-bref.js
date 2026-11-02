#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const input = fs.readFileSync(process.argv[2], "utf8");
const labels = new Map();
const atoms = [];
let offset = 0;

for (const rawLine of input.split(/\n/)) {
  const line = rawLine.replace(/#.*/, "").trim();
  if (!line) continue;
  for (const tok of line.split(/\s+/)) {
    if (tok.startsWith(":")) {
      labels.set(tok.slice(1), offset);
      continue;
    }
    if (tok.startsWith("@") || tok.startsWith("!")) {
      atoms.push({
        kind: tok[0] === "@" ? "b" : "bl",
        label: tok.slice(1),
        offset,
      });
      offset += 4;
      continue;
    }
    if (!/^[0-9a-fA-F]{2}$/.test(tok)) {
      throw new Error(`bad token: ${tok}`);
    }
    atoms.push({ kind: "byte", value: parseInt(tok, 16), offset });
    offset += 1;
  }
}

const out = Buffer.alloc(offset);
for (const atom of atoms) {
  if (atom.kind === "byte") {
    out[atom.offset] = atom.value;
    continue;
  }

  const target = labels.get(atom.label);
  if (target === undefined) throw new Error(`unknown label: ${atom.label}`);
  const delta = target - atom.offset;
  if (delta % 4 !== 0) throw new Error(`unaligned branch to ${atom.label}`);
  const imm26 = delta >> 2;
  if (imm26 < -(1 << 25) || imm26 >= (1 << 25)) {
    throw new Error(`branch out of range to ${atom.label}`);
  }
  const opcode = atom.kind === "b" ? 0x14000000 : 0x94000000;
  out.writeUInt32LE((opcode | (imm26 & 0x03ffffff)) >>> 0, atom.offset);
}

const fixtureName = path.basename(process.argv[2]);
let expected;
if (fixtureName === "test-b-back.hex1" || fixtureName === "test-bl-back.hex1") {
  const expectedLastWord = fixtureName === "test-bl-back.hex1" ? 0x97fffffa : 0x17fffffa;
  expected = Buffer.from([
    0x40, 0x05, 0x80, 0xd2,
    0x30, 0x00, 0x80, 0xd2,
    0x01, 0x10, 0x00, 0xd4,
    0x20, 0x00, 0x80, 0xd2,
    0x30, 0x00, 0x80, 0xd2,
    0x01, 0x10, 0x00, 0xd4,
    expectedLastWord & 0xff,
    (expectedLastWord >>> 8) & 0xff,
    (expectedLastWord >>> 16) & 0xff,
    (expectedLastWord >>> 24) & 0xff,
  ]);
} else {
  const expectedFirstWord = fixtureName === "test-bl.hex1" ? 0x94000004 : 0x14000004;
  expected = Buffer.alloc(28);
  expected.writeUInt32LE(expectedFirstWord, 0);
  Buffer.from([
    0x20, 0x00, 0x80, 0xd2,
    0x30, 0x00, 0x80, 0xd2,
    0x01, 0x10, 0x00, 0xd4,
    0x40, 0x05, 0x80, 0xd2,
    0x30, 0x00, 0x80, 0xd2,
    0x01, 0x10, 0x00, 0xd4,
  ]).copy(expected, 4);
}

if (!out.equals(expected)) {
  process.stderr.write(`arm64 branch fixture mismatch\n`);
  process.stderr.write(`got      ${out.toString("hex")}\n`);
  process.stderr.write(`expected ${expected.toString("hex")}\n`);
  process.exit(1);
}

const outName = path.basename(process.argv[2], ".hex1");
const outPath = path.join(path.dirname(process.argv[2]), "..", "build", `${outName}.bin`);
fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, out);
console.log(`${path.basename(process.argv[2])}: arm64 label reference verified`);
