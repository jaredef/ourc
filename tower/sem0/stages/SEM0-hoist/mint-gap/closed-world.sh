#!/usr/bin/env bash
# C-RZIR-CLOSED-WORLD: in a complete rzir module every call target must be DEFINED in that module.
#
# This is the fail-closed predicate the substrate already satisfies but does not enforce. It needs no builtin
# allowlist and no `__` convention: the runtime prelude is emitted as ordinary `fn` definitions, so the set of
# backend-provided symbols is EMPTY and the rule is simply  called subset-of defined.
#
# Measured 2026-08-28: every tower image satisfies it exactly (self-image 0 undefined of 892 called; gen2 0;
# gen3 0; L4 0), while the dev image violates it by exactly ONE symbol and every fail-open probe violates it.
# That separation is what makes it a gate rather than a heuristic -- there is nothing to tune.
#
#   bash closed-world.sh <file.rzir> [...]        # exits 1 if any module is not closed
set -uo pipefail
[ $# -ge 1 ] || { echo "usage: closed-world.sh <file.rzir> [...]"; exit 2; }
rc=0
for f in "$@"; do
  [ -s "$f" ] || { echo "  $(basename "$f"): EMPTY OR MISSING -- a module that does not exist is not 'closed'"; rc=1; continue; }
  c="$(mktemp)"; d="$(mktemp)"
  grep -oE '= call [A-Za-z_][A-Za-z0-9_:]*' "$f" | awk '{print $3}' | sort -u > "$c"
  grep -oE '^fn [A-Za-z_][A-Za-z0-9_:]*'    "$f" | awk '{print $2}' | sort -u > "$d"
  u="$(comm -23 "$c" "$d")"; n="$(printf '%s' "$u" | grep -c . || true)"
  printf '  %-30s called=%-5s defined=%-5s undefined=%s\n' "$(basename "$f")" "$(wc -l < "$c")" "$(wc -l < "$d")" "$n"
  if [ "$n" -ne 0 ]; then printf '%s\n' "$u" | sed 's/^/      UNDEFINED: /'; rc=1; fi
  rm -f "$c" "$d"
done
[ $rc -eq 0 ] && echo "== all modules closed ==" || echo "== NOT CLOSED: the emitter produced calls to symbols that do not exist =="
exit $rc
