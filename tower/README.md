# tower/ — the self-hosting compiler tower

This directory holds the **produced chain** of the rusteze self-hosting tower plus every gate
that verifies it. It was extracted from `apparatus/arcs/` (2026-08-27) so the buildable,
trust-rooted tower is a first-class subtree, separate from the derivation history that remains
in the arcs.

## Build & verify

```
SP=/tmp/tw bash tower/build-tower.sh          # build the tower FROM THE HAND-HEX + the tower gates
SP=/tmp/tw bash tower/build-tower.sh --gates  # ... + every sub-arc gate (S1..S9) + the trust gate
SP=/tmp/tw bash tower/build-tower.sh --dev    # opt-in fast loop: seed from the committed dev blobs
```

`$SP` is a scratch dir (may be empty/nonexistent): `lib/scratch.sh`'s `sp_seed` mints the two
roots from in-repo source. No host cc/ld enters the produced chain.

## Layout (Phase 2a: clean tier names)

The tower spine now uses clean tier names under `tower/`:

- `arcs/L0` — hand-hex seed → hex1/hex2 → M0 assembler + cf.m0 (trust root); `chain-proof.sh`
- `arcs/L1` — `c-to-rzir.m0` (hex-rooted M0 C front-end)
- `arcs/L2` — `ccc.c` self-hosting C compiler + the C backend stages
- `arcs/L3` — `rzrs.c` C-seed Rust front-end
- `arcs/L4` — L4 + `bootstrap.sh`
- `arcs/L5` — L5 (frozen mainline) + `cf.hoist.seed` + `selfhost.sh`
- `arcs/L6` — L6 (live dev tier) + `stage.sh`
- `arcs/backend-cf` — `cf.rs` (backend codegen, Rust)
- `arcs/backend-toolchain` — `rzir-{ld,opt,ra}.rs` + **`provenance.sh`** (the trust gate)
- `arcs/backend-live` — `build-live-backend.sh` (mints the all-Rust twins)
- `arcs/sem0` — SEM0 / `s1_parser.rs` + the S1..S9/D0 sub-arc gates
- `arcs/parity` — the 90-program parity gate
- `arcs/corpus` — the parity corpus gate (r2-corpus)
- `arcs/census` — the whole-crate census gate (oracle-gated)
- `arcs/tcc`, `arcs/rust-closure` — helpers/test sets
- `lib/scratch.sh`, `build-tower.sh`, `promote.sh`, `README.md`

Structure within each tier is preserved, so relative refs resolve unchanged. `tower/` sits at
repo-root depth (same as `apparatus/`).

## Trust

`provenance.sh` is the trust gate: it re-derives the whole chain from the hand-hex in-gate and
DDC-anchors it (Wheeler diverse double compilation). See
`apparatus/docs/tower-naming-and-trust-path.md`. The `--gates` run ends on it.

## Remaining cleanup (planned)

Prune the non-gate derivation files (docs, promotions/, *.pre/*.out, cargo target/, instrument
boards) so `tower/` holds only the buildable tower + its gates. Gate-verified.
