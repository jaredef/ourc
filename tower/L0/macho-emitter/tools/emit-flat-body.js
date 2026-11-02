#!/usr/bin/env node

const fs = require("fs");
const { resolveFlatStatus } = require("./flat-status");

const input = fs.readFileSync(0, "utf8");
const status = resolveFlatStatus(input);

const code = Buffer.alloc(12);
code.writeUInt32LE((0xd2800000 | ((status & 0xffff) << 5)) >>> 0, 0); // mov x0, #status
code.writeUInt32LE(0xd2800030, 4); // mov x16, #1 ; exit
code.writeUInt32LE(0xd4001001, 8); // svc #0x80

process.stdout.write(code);
