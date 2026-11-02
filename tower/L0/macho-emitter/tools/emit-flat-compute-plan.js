#!/usr/bin/env node
const fs = require("fs");
const { computeTokens } = require("./flat-compute-plan");

const input = fs.readFileSync(0, "utf8");
process.stdout.write(`${computeTokens(input).join("\n")}\n`);
