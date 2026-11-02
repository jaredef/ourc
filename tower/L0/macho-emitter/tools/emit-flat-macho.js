#!/usr/bin/env node

const childProcess = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const input = fs.readFileSync(0);
const code = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-flat-body.js")],
  { input }
);

const dir = fs.mkdtempSync(path.join(os.tmpdir(), "rusteze-macho-emitter-"));
try {
  const codePath = path.join(dir, "body.bin");
  const outPath = path.join(dir, "out.macho");
  fs.writeFileSync(codePath, code);
  childProcess.execFileSync(
    process.execPath,
    [
      path.join(__dirname, "..", "..", "macho-hex0", "tools", "make-macho-hex0.js"),
      "--wrap-code",
      codePath,
      outPath,
      "rusteze-rzir-macho-flat",
    ],
    { stdio: ["ignore", "ignore", "inherit"] }
  );
  process.stdout.write(fs.readFileSync(outPath));
} finally {
  fs.rmSync(dir, { recursive: true, force: true });
}
