#!/usr/bin/env bash
# oracle-vendor-setup.sh -- provision the (B) FULL-BUILD oracle: the official rustc-1.95.0 source tree (submodules
# bundled) + bootstrap.toml + a baseline `x.py check compiler` (downloads stage0 + prebuilt LLVM, checks the whole
# compiler). Idempotent. ORACLE-class, measurement-only, never in the produced chain. Heavy first run (~a few min
# + downloads); a no-op once provisioned. Skips cleanly if it cannot provision (offline).
set -uo pipefail
ORACLE_SRC="${ORACLE_SRC:-/tmp/rustc-src/rustc-1.95.0-src}"; TAG=1.95.0
mkdir -p "$(dirname "$ORACLE_SRC")"
if [ -f "$ORACLE_SRC/.oracle.checked" ]; then echo "oracle (B) provisioned"; exit 0; fi
if [ ! -f "$ORACLE_SRC/x.py" ]; then
  cd "$(dirname "$ORACLE_SRC")"
  [ -f "rustc-$TAG-src.tar.gz" ] || timeout 900 curl -sL "https://static.rust-lang.org/dist/rustc-$TAG-src.tar.gz" -o "rustc-$TAG-src.tar.gz" || { echo "SKIP: cannot fetch src"; exit 0; }
  timeout 600 tar xzf "rustc-$TAG-src.tar.gz" || { echo "SKIP: extract failed"; exit 0; }
fi
[ -f "$ORACLE_SRC/bootstrap.toml" ] || cat > "$ORACLE_SRC/bootstrap.toml" <<'TOML'
change-id = "ignore"
[rust]
channel = "nightly"
deny-warnings = false
[build]
docs = false
[llvm]
download-ci-llvm = true
TOML
( cd "$ORACLE_SRC" && timeout 3600 python3 x.py check compiler ) >/dev/null 2>&1 && touch "$ORACLE_SRC/.oracle.checked" \
  && echo "oracle (B) baseline: x.py check compiler PASSED" || echo "SKIP: x.py check did not pass"
