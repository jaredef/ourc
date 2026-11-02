#!/usr/bin/env node

const fs = require("fs");

const input = fs.readFileSync(0, "utf8");
const labels = new Map();

function isHex(ch) {
  return (ch >= "0" && ch <= "9") || (ch >= "a" && ch <= "f");
}

function hexValue(ch) {
  if (ch >= "0" && ch <= "9") return ch.charCodeAt(0) - 0x30;
  return ch.charCodeAt(0) - 0x57;
}

function pass(emit) {
  const out = [];
  let pending = -1;
  let offset = 0;

  for (let i = 0; i < input.length; i += 1) {
    const ch = input[i];
    if (ch === "#") {
      while (i < input.length && input[i] !== "\n") i += 1;
      continue;
    }
    if (ch === ":") {
      i += 1;
      if (i >= input.length) throw new Error("label marker missing name");
      if (!emit) labels.set(input[i], offset);
      continue;
    }
    if (ch === "@" || ch === "!") {
      i += 1;
      if (i >= input.length) throw new Error(`${ch} marker missing name`);
      const label = input[i];
      if (emit) {
        const target = labels.get(label);
        if (target === undefined) throw new Error(`unknown label: ${label}`);
        const delta = target - offset;
        if (delta % 4 !== 0) throw new Error(`unaligned branch: ${label}`);
        const imm26 = delta >> 2;
        if (imm26 < -(1 << 25) || imm26 >= (1 << 25)) {
          throw new Error(`branch out of range: ${label}`);
        }
        const opcode = ch === "@" ? 0x14000000 : 0x94000000;
        const word = (opcode | (imm26 & 0x03ffffff)) >>> 0;
        out.push(word & 0xff, (word >>> 8) & 0xff, (word >>> 16) & 0xff, (word >>> 24) & 0xff);
      }
      offset += 4;
      continue;
    }
    if (!isHex(ch)) continue;
    const nibble = hexValue(ch);
    if (pending < 0) {
      pending = nibble;
      continue;
    }
    if (emit) out.push((pending << 4) | nibble);
    pending = -1;
    offset += 1;
  }

  return Buffer.from(out);
}

pass(false);
process.stdout.write(pass(true));
