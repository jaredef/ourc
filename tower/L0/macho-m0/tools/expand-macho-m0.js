#!/usr/bin/env node

const fs = require("fs");

const TEXT_BASE = 0x100000000n;
const TEXT_CODE_OFFSET = 0x2b0n;

const input = fs.readFileSync(0, "utf8");
const tokens = [];

for (const line of input.split(/\n/)) {
  const cut = line.replace(/#.*/, "");
  for (const token of cut.trim().split(/\s+/)) {
    if (token) tokens.push(token);
  }
}

const labels = new Map();
const macros = new Map();

function isHexToken(token) {
  return token.length > 0 && token.length % 2 === 0 && /^[0-9a-fA-F]+$/.test(token);
}

function hexBytes(token) {
  if (!isHexToken(token)) throw new Error(`invalid hex token: ${token}`);
  return Buffer.from(token, "hex");
}

function u32le(value) {
  return [
    value & 0xff,
    (value >>> 8) & 0xff,
    (value >>> 16) & 0xff,
    (value >>> 24) & 0xff,
  ];
}

function u64le(value) {
  const bytes = [];
  for (let i = 0n; i < 8n; i += 1n) {
    bytes.push(Number((value >> (i * 8n)) & 0xffn));
  }
  return bytes;
}

function refSize(kind) {
  if (kind === "?") return 4;
  if (kind === "=") return 8;
  return 4;
}

function emitRef(kind, name, offset, wordToken) {
  const target = labels.get(name);
  if (target === undefined) throw new Error(`unknown label: ${name}`);
  if (kind === "&") return u32le(target >>> 0);
  if (kind === "=") return u64le(TEXT_BASE + TEXT_CODE_OFFSET + BigInt(target));

  const delta = target - offset;
  if (delta % 4 !== 0) throw new Error(`unaligned branch: ${name}`);
  if (kind === "?") {
    const imm19 = delta >> 2;
    if (imm19 < -(1 << 18) || imm19 >= (1 << 18)) {
      throw new Error(`imm19 branch out of range: ${name}`);
    }
    const base = Number.parseInt(wordToken, 16) >>> 0;
    const word = ((base & ~0x00ffffe0) | ((imm19 & 0x7ffff) << 5)) >>> 0;
    return u32le(word);
  }

  const imm26 = delta >> 2;
  if (imm26 < -(1 << 25) || imm26 >= (1 << 25)) {
    throw new Error(`branch out of range: ${name}`);
  }
  const opcode = kind === "@" ? 0x14000000 : 0x94000000;
  const word = (opcode | (imm26 & 0x03ffffff)) >>> 0;
  return u32le(word);
}

function pass(emit) {
  const out = [];
  let offset = 0;

  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i];
    const head = token[0];

    if (head === "%") {
      const name = token.slice(1);
      const body = tokens[++i];
      if (!name || body === undefined || !isHexToken(body)) {
        throw new Error(`invalid macro definition: ${token}`);
      }
      if (!emit) macros.set(name, [...hexBytes(body)]);
      continue;
    }

    if (head === ":") {
      const name = token.slice(1);
      if (!name) throw new Error("empty label");
      if (!emit) labels.set(name, offset);
      continue;
    }

    if (head === "@" || head === "!" || head === "&" || head === "=") {
      const name = token.slice(1);
      if (!name) throw new Error(`empty reference: ${token}`);
      if (emit) out.push(...emitRef(head, name, offset));
      offset += refSize(head);
      continue;
    }

    if (head === "?") {
      const colon = token.indexOf(":");
      const word = token.slice(1, colon);
      const name = colon >= 0 ? token.slice(colon + 1) : "";
      if (colon < 0 || !/^[0-9a-f]{8}$/.test(word) || !name) {
        throw new Error(`invalid imm19 reference: ${token}`);
      }
      if (emit) out.push(...emitRef(head, name, offset, word));
      offset += refSize(head);
      continue;
    }

    const macro = macros.get(token);
    if (macro) {
      if (emit) out.push(...macro);
      offset += macro.length;
      continue;
    }

    if (!isHexToken(token)) {
      const error = new Error(`unknown token: ${token}`);
      error.exitCode = 3;
      throw error;
    }
    const bytes = [...hexBytes(token)];
    if (emit) out.push(...bytes);
    offset += bytes.length;
  }

  return Buffer.from(out);
}

try {
  pass(false);
  process.stdout.write(pass(true));
} catch (error) {
  process.stderr.write(`${error.message}\n`);
  process.exit(error.exitCode || 1);
}
