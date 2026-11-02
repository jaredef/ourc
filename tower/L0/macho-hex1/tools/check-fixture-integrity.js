#!/usr/bin/env node

const fs = require("fs");

const file = process.argv[2];
if (!file) {
  process.stderr.write("usage: check-fixture-integrity.js <fixture.hex1>\n");
  process.exit(2);
}

const input = fs.readFileSync(file, "utf8");
const labels = new Map();
const refs = [];
let offset = 0;
let pending = -1;

function fail(message) {
  process.stderr.write(`${file}: ${message}\n`);
  process.exit(1);
}

function isHex(ch) {
  return (ch >= "0" && ch <= "9") || (ch >= "a" && ch <= "f");
}

function isUpperHex(ch) {
  return ch >= "A" && ch <= "F";
}

function nextMeaningful(index) {
  for (let i = index; i < input.length; i += 1) {
    const ch = input[i];
    if (ch === "#") {
      while (i < input.length && input[i] !== "\n") i += 1;
      continue;
    }
    if (/\s/.test(ch)) continue;
    return [i, ch];
  }
  return [-1, ""];
}

for (let i = 0; i < input.length; i += 1) {
  const ch = input[i];
  if (ch === "#") {
    while (i < input.length && input[i] !== "\n") i += 1;
    continue;
  }
  if (ch === ":" || ch === "@" || ch === "!") {
    const [tailIndex, tail] = nextMeaningful(i + 1);
    if (tailIndex < 0) fail(`${ch} marker missing label tail`);
    if (tail === ":" || tail === "@" || tail === "!" || tail === "#") {
      fail(`${ch} marker has invalid label tail`);
    }
    i = tailIndex;
    if (ch === ":") {
      labels.set(tail, offset);
    } else {
      refs.push({ kind: ch, label: tail, offset });
      offset += 4;
    }
    continue;
  }
  if (isUpperHex(ch)) fail("uppercase hex is outside this seed mouth");
  if (!isHex(ch)) continue;
  if (pending < 0) {
    pending = 1;
  } else {
    pending = -1;
    offset += 1;
  }
}

if (pending >= 0) fail("odd trailing hex nibble");

for (const ref of refs) {
  if (!labels.has(ref.label)) fail(`${ref.kind}${ref.label} references unknown label`);
  const target = labels.get(ref.label);
  const delta = target - ref.offset;
  if (delta % 4 !== 0) fail(`${ref.kind}${ref.label} target is not 4-byte aligned`);
  const imm26 = delta >> 2;
  if (imm26 < -(1 << 25) || imm26 >= (1 << 25)) {
    fail(`${ref.kind}${ref.label} target is out of arm64 imm26 range`);
  }
}
