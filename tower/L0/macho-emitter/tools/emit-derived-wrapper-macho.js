#!/usr/bin/env node
const childProcess = require("child_process");
const fs = require("fs");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: emit-derived-wrapper-macho.js <body> <identifier>\n");
  process.exit(1);
}

function runTool(name) {
  return childProcess.execFileSync(
    process.execPath,
    [path.join(__dirname, name), bodyPath, identifier]
  );
}

const prefix = runTool("emit-derived-wrapper-prefix.js");
const body = fs.readFileSync(bodyPath);
const linkeditPrefix = runTool("emit-derived-wrapper-linkedit-prefix.js");
const codeSignature = runTool("emit-derived-wrapper-code-signature.js");

process.stdout.write(Buffer.concat([prefix, body, linkeditPrefix, codeSignature]));
