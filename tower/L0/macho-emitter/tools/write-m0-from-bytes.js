#!/usr/bin/env node
const fs = require("fs");

const title = process.argv[2] || "M0-authored byte body";
const bytes = fs.readFileSync(0);

process.stdout.write(`# ${title}\n`);
for (let i = 0; i < bytes.length; i += 16) {
  process.stdout.write(bytes.subarray(i, i + 16).toString("hex").match(/../g).join(" "));
  process.stdout.write("\n");
}
