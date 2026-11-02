#!/usr/bin/env node
const { resolveFlatStatus } = require("./flat-status");

const fs = require("fs");
const input = fs.readFileSync(0, "utf8");
const status = resolveFlatStatus(input);
process.stdout.write(Buffer.from([status]));
