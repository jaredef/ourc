#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { resolveFlatStatus } = require("./flat-status");

const fixturesDir = path.join(__dirname, "..", "..", "..", "..", "..", "locales", "stage3-ir-backend", "fixtures");
const cases = [
  ["add", 11],
  ["sub", 38],
  ["renamed-temps", 12],
  ["chained-op", 7],
  ["threeconst", 15],
  ["wide-temps", 7],
];

for (const [name, expected] of cases) {
  const source = fs.readFileSync(path.join(fixturesDir, `${name}.rzir`), "utf8");
  const actual = resolveFlatStatus(source);
  if (actual !== expected) {
    process.stderr.write(`${name}: status ${actual}, expected ${expected}\n`);
    process.exit(1);
  }
}

process.stdout.write("flat status resolver verified: add/sub/renamed/chained/threeconst/wide-temps\n");
