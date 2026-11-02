#!/usr/bin/env node

const childProcess = require("child_process");
const fs = require("fs");
const path = require("path");
const { resolveFlatStatus } = require("./flat-status");

const input = fs.readFileSync(0, "utf8");
const status = resolveFlatStatus(input);
const movWord = (0xd2800000 | ((status & 0xffff) << 5)) >>> 0;
const movHex = Buffer.alloc(4);
movHex.writeUInt32LE(movWord, 0);

const source = [
  fs.readFileSync(path.join(__dirname, "..", "body-templates", "lib.m0"), "utf8"),
  `%mov_x0_status ${movHex.toString("hex")}`,
  "mov_x0_status",
  "mov_x16_exit",
  "svc_0x80",
  "",
].join("\n");

const body = childProcess.execFileSync(
  path.join(__dirname, "..", "..", "macho-m0", "macho-m0.bin"),
  { input: source }
);

process.stdout.write(body);

