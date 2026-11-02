#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const dirs = [...new Set(process.argv.slice(2).map((dir) => path.resolve(dir)))];
if (dirs.length < 2) {
  process.stderr.write("usage: compare-wrapper-templates.js <template-dir> <template-dir>...\n");
  process.exit(1);
}

function readTemplate(dir) {
  return {
    dir,
    slot: JSON.parse(fs.readFileSync(path.join(dir, "slot.json"), "utf8")),
    prefix: fs.readFileSync(path.join(dir, "prefix.bin")),
    suffix: fs.readFileSync(path.join(dir, "suffix.bin")),
  };
}

function diffRanges(buffers) {
  const max = Math.max(...buffers.map((buffer) => buffer.length));
  const ranges = [];
  let start = null;
  for (let i = 0; i < max; i += 1) {
    const first = i < buffers[0].length ? buffers[0][i] : undefined;
    const differs = buffers.some((buffer) => (i < buffer.length ? buffer[i] : undefined) !== first);
    if (differs && start === null) start = i;
    if (!differs && start !== null) {
      ranges.push([start, i - 1]);
      start = null;
    }
  }
  if (start !== null) ranges.push([start, max - 1]);
  return ranges;
}

const templates = dirs.map(readTemplate);
const prefixLengths = [...new Set(templates.map((template) => template.prefix.length))];
const suffixLengths = [...new Set(templates.map((template) => template.suffix.length))];
const bodyOffsets = [...new Set(templates.map((template) => template.slot.bodyOffset))];
const bodyLengths = [...new Set(templates.map((template) => template.slot.bodyBytes))].sort((a, b) => a - b);
const machoLengths = [...new Set(templates.map((template) => template.slot.machoBytes))].sort((a, b) => a - b);

const result = {
  count: templates.length,
  bodyOffsets,
  bodyLengths,
  machoLengths,
  prefixLengths,
  suffixLengths,
  prefixDiffRanges: diffRanges(templates.map((template) => template.prefix)),
  suffixDiffRanges: diffRanges(templates.map((template) => template.suffix)),
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
