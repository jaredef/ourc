# s1-build.sh -- SOURCED by the S-stage gates (expects P, SP, CC; honours SEM0_CC). Builds the two compilers every
# S-gate differentials against -- $SP/s1.rc (rustc -O $P) and $SP/s1.rz (SEM0_CC -> ld|opt|ra|cf.hoist) -- ONCE per
# set of inputs, and reuses them while the inputs are unchanged. Before 2026-10-26 each of 13 gate scripts rebuilt
# both itself (~10 min apiece with the tower-built SEM0_CC): the same binaries, from the same inputs, to the same
# paths, ~2 hours of every --gates run.
#
# Cache key ($SP/.s1.key) = sha256 CONTENT hashes only (each gate reaches $P by its own ../ path, so path text in
# the key made every gate miss -- fixed 2026-10-26) of: the source, the SEM0_CC wrapper and every file it names (the frozen binary it
# execs), the backend tools, plus `rustc -V`. Any change rebuilds. The key is written only after both builds
# succeed, so a failed or interrupted build is never reused. S1_NOCACHE=1 forces a rebuild.
s1_key(){
  local w="${SEM0_CC:-$SP/sem0.cc}" f
  sha256sum "$P" "$w" "$SP/ld.bin" "$CC/opt/rzir-opt.bin" "$CC/ra/rzir-ra.bin" "$SP/cf.hoist" 2>&1 | cut -d" " -f1
  grep -oE '/[^" ]+' "$w" 2>/dev/null | sort -u | while read -r f; do [ -f "$f" ] && sha256sum < "$f" | cut -d" " -f1; done
  rustc -V 2>&1
}
s1_key > "$SP/.s1.key.new"
if [ "${S1_NOCACHE:-0}" != 1 ] && [ -x "$SP/s1.rc" ] && [ -s "$SP/s1.rz" ] && cmp -s "$SP/.s1.key.new" "$SP/.s1.key" 2>/dev/null; then
  echo "  [s1.rc + s1.rz reused: inputs unchanged ($(sha256sum "$SP/.s1.key" | cut -c1-12))]" >&2
else
  rm -f "$SP/.s1.key"
  rustc --edition 2021 -O "$P" -o "$SP/s1.rc" 2>/dev/null || { echo "FAIL: rustc build of s1_parser"; exit 1; }
  "${SEM0_CC:-$SP/sem0.cc}" < "$P" 2>/dev/null | "$SP/ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$SP/cf.hoist" > "$SP/s1.rz" 2>/dev/null; chmod +x "$SP/s1.rz"
  [ -s "$SP/s1.rz" ] && mv "$SP/.s1.key.new" "$SP/.s1.key"
fi
rm -f "$SP/.s1.key.new"
