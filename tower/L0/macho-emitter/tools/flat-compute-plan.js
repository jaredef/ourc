function parseFlat(source) {
  const ops = [];
  let ret = null;
  for (const raw of source.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line === "end" || line.startsWith("ir ")) continue;
    let m = line.match(/^v([0-9]): i32 = const ([0-9]+)$/);
    if (m) {
      ops.push({ kind: "const", dst: Number(m[1]), value: Number(m[2]) });
      continue;
    }
    m = line.match(/^v([0-9]): i32 = add v([0-9]) v([0-9])$/);
    if (m) {
      ops.push({ kind: "add", dst: Number(m[1]), a: Number(m[2]), b: Number(m[3]) });
      continue;
    }
    m = line.match(/^v([0-9]): i32 = sub v([0-9]) v([0-9])$/);
    if (m) {
      ops.push({ kind: "sub", dst: Number(m[1]), a: Number(m[2]), b: Number(m[3]) });
      continue;
    }
    m = line.match(/^ret v([0-9])$/);
    if (m) {
      ret = Number(m[1]);
      continue;
    }
    throw new Error(`unsupported flat rzir line: ${line}`);
  }
  if (ret === null) throw new Error("missing ret");
  return { ops, ret };
}

function opToken(op) {
  if (op.kind === "const") return `mov_x${op.dst}_${op.value}`;
  return `${op.kind}_x${op.dst}_x${op.a}_x${op.b}`;
}

function computeTokens(source) {
  const { ops, ret } = parseFlat(source);
  return [
    ...ops.map(opToken),
    `mov_x0_x${ret}`,
    "mov_x16_exit",
    "svc_0x80",
  ];
}

module.exports = { computeTokens, parseFlat };
