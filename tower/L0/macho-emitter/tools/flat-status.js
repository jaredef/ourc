"use strict";

function resolveFlatStatus(input) {
  const values = new Map();
  let retTemp = null;

  function tempValue(name) {
    if (!values.has(name)) throw new Error(`unknown temp: ${name}`);
    return values.get(name);
  }

  for (const rawLine of input.split(/\n/)) {
    const line = rawLine.trim();
    if (!line || line === "end" || line.startsWith("ir ")) continue;

    let match = line.match(/^(v\d+):\s+i32\s+=\s+const\s+(-?\d+)$/);
    if (match) {
      values.set(match[1], Number(match[2]) | 0);
      continue;
    }

    match = line.match(/^(v\d+):\s+i32\s+=\s+(add|sub)\s+(v\d+)\s+(v\d+)$/);
    if (match) {
      const left = tempValue(match[3]);
      const right = tempValue(match[4]);
      values.set(match[1], match[2] === "add" ? (left + right) | 0 : (left - right) | 0);
      continue;
    }

    match = line.match(/^ret\s+(v\d+)$/);
    if (match) {
      retTemp = match[1];
      continue;
    }
  }

  if (!retTemp) throw new Error("missing ret");
  return tempValue(retTemp) & 0xff;
}

module.exports = { resolveFlatStatus };

