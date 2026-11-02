#!/usr/bin/env node
const childProcess = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { computeTokens } = require("./flat-compute-plan");

const input = fs.readFileSync(0);

function toM0(source) {
  const lines = [
    "# Generated M0 source for flat rzir compute body",
    fs.readFileSync(path.join(__dirname, "..", "compute-templates", "lib.m0"), "utf8").trimEnd(),
    "",
  ];
  lines.push(...computeTokens(source));
  lines.push("");
  return lines.join("\n");
}

const dir = fs.mkdtempSync(path.join(os.tmpdir(), "rusteze-compute-body-m0-"));
try {
  const sourcePath = path.join(dir, "compute.m0");
  const source = toM0(input.toString("utf8"));
  if (process.argv.includes("--emit-source")) {
    process.stdout.write(source);
    process.exit(0);
  }
  fs.writeFileSync(sourcePath, source);
  const m0Path = path.join(__dirname, "..", "..", "macho-m0", "macho-m0.bin");
  const expanded = childProcess.execFileSync(m0Path, { input: fs.readFileSync(sourcePath) });
  process.stdout.write(expanded);
} finally {
  fs.rmSync(dir, { recursive: true, force: true });
}
