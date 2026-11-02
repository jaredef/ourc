#!/bin/bash
# fmt.sh -- format ourc's crates with rustfmt (config: ourc/rustfmt.toml).
#   bash ourc/fmt.sh            format ourc/src/*.rs in place
#   bash ourc/fmt.sh --check    exit 1 if any file is not formatted (no changes made)
# Formatting must stay semantics-free: after a reformat, rebuild ourc and confirm every probe's output is
# unchanged (the 2026-10-28 reformat compared all 575 surface/cli programs byte-for-byte).
set -eu
D="$(cd "$(dirname "$0")" && pwd)"
if [ "${1:-}" = "--check" ]; then
  rustfmt --edition 2021 --config-path "$D/rustfmt.toml" --check "$D"/src/*.rs >/dev/null && { echo "ourc fmt: clean"; exit 0; }
  echo "ourc fmt: NOT formatted (run: bash ourc/fmt.sh)"; exit 1
fi
# rustfmt is not idempotent around some trailing /* */ comments (a 2nd pass re-indents them): repeat to a fixed point
for i in 1 2 3 4; do
  rustfmt --edition 2021 --config-path "$D/rustfmt.toml" "$D"/src/*.rs
  rustfmt --edition 2021 --config-path "$D/rustfmt.toml" --check "$D"/src/*.rs >/dev/null 2>&1 && break
done
echo "ourc fmt: formatted $(ls "$D"/src/*.rs | wc -l) files (fixed point after $i pass(es))"
