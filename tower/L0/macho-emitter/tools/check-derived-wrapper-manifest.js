#!/usr/bin/env node
const fs = require("fs");

const manifestPath = process.argv[2];
const fieldsPath = process.argv[3];
const layoutPath = process.argv[4];
if (!manifestPath || !fieldsPath || !layoutPath) {
  process.stderr.write("usage: check-derived-wrapper-manifest.js <manifest> <wrapper-fields-json> <derived-layout-json>\n");
  process.exit(1);
}

const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
const fields = JSON.parse(fs.readFileSync(fieldsPath, "utf8"));
const layout = JSON.parse(fs.readFileSync(layoutPath, "utf8"));

function fail(path, actual, expected) {
  process.stderr.write(`${path}: got ${actual}, expected ${expected}\n`);
  process.exit(1);
}

function eq(path, actual, expected) {
  if (actual !== expected) fail(path, actual, expected);
}

eq("version", manifest.version, 1);
eq("body.bytes", manifest.body.bytes, fields.body.bytes);
eq("body.offset", manifest.body.offset, fields.body.offset);
eq("body.address", manifest.body.address, fields.textSection.addr);
eq("layout.sizeofcmds", manifest.layout.sizeofcmds, fields.header.sizeofcmds);
eq("layout.loadCommandsEnd", manifest.layout.loadCommandsEnd, fields.header.loadCommandsEnd);
eq("layout.machoBytes", manifest.layout.machoBytes, fields.machoBytes);
eq("layout.codeSignatureOff", manifest.layout.codeSignatureOff, fields.codeSignature.dataoff);
eq("layout.codeSignatureSize", manifest.layout.codeSignatureSize, fields.codeSignature.datasize);
eq("layout.linkeditOffset", manifest.layout.linkeditOffset, layout.linkedit.offset);
eq("layout.linkeditSize", manifest.layout.linkeditSize, layout.linkedit.size);
eq("codeDirectory.length", manifest.codeDirectory.length, layout.codeSignature.codeDirectoryLength);

if (!Array.isArray(manifest.codeDirectory.pageHashes) || manifest.codeDirectory.pageHashes.length !== manifest.codeDirectory.nCodeSlots) {
  process.stderr.write("page hash count does not match nCodeSlots\n");
  process.exit(1);
}

for (const hash of manifest.codeDirectory.pageHashes) {
  if (!/^[0-9a-f]{64}$/.test(hash)) {
    process.stderr.write(`invalid page hash: ${hash}\n`);
    process.exit(1);
  }
}

process.stdout.write("manifest verified\n");
