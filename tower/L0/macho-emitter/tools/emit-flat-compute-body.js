#!/usr/bin/env node
const fs = require("fs");

function u32le(word) {
  const buffer = Buffer.alloc(4);
  buffer.writeUInt32LE(word >>> 0, 0);
  return buffer;
}

function movImm(rd, imm) {
  if (imm < 0 || imm > 0xffff) {
    throw new Error(`immediate out of first-slice range: ${imm}`);
  }
  return 0xd2800000 | ((imm & 0xffff) << 5) | rd;
}

function addReg(rd, rn, rm) {
  return 0x8b000000 | (rm << 16) | (rn << 5) | rd;
}

function subReg(rd, rn, rm) {
  return 0xcb000000 | (rm << 16) | (rn << 5) | rd;
}

function movReg(rd, rm) {
  return 0xaa0003e0 | (rm << 16) | rd;
}

function parseFlat(input) {
  const ops = [];
  let ret = null;
  for (const raw of input.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line === "end" || line.startsWith("ir ")) continue;
    let m = line.match(/^v([0-9]): i32 = const ([0-9]+)$/);
    if (m) {
      ops.push({ kind: "const", dst: Number(m[1]), value: Number(m[2]) });
      continue;
    }
    m = line.match(/^v([0-9]): i32 = add v([0-9]) v([0-9])$/);
    if (m) {
      ops.push({ kind: "add", dst: Number(m[1]), a: Number(m[2]), b: Number(m[3]) });
      continue;
    }
    m = line.match(/^v([0-9]): i32 = sub v([0-9]) v([0-9])$/);
    if (m) {
      ops.push({ kind: "sub", dst: Number(m[1]), a: Number(m[2]), b: Number(m[3]) });
      continue;
    }
    m = line.match(/^ret v([0-9])$/);
    if (m) {
      ret = Number(m[1]);
      continue;
    }
    throw new Error(`unsupported flat rzir line: ${line}`);
  }
  if (ret === null) throw new Error("missing ret");
  return { ops, ret };
}

const input = fs.readFileSync(0, "utf8");
const { ops, ret } = parseFlat(input);
const words = [];
for (const op of ops) {
  if (op.kind === "const") words.push(movImm(op.dst, op.value));
  else if (op.kind === "add") words.push(addReg(op.dst, op.a, op.b));
  else if (op.kind === "sub") words.push(subReg(op.dst, op.a, op.b));
}
words.push(movReg(0, ret));
words.push(0xd2800030); // mov x16, #1 ; exit
words.push(0xd4001001); // svc #0x80

process.stdout.write(Buffer.concat(words.map(u32le)));
