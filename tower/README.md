# tower/

The compiler tower: every rung's source, from the hand-written hex seed up to SEM0, plus the gates that verify it.

## Build and verify

```
SP=<scratch> bash tower/build-tower.sh          # build the tower from the hand-hex
SP=<scratch> bash tower/build-tower.sh --gates  # ... and run every gate, ending on the trust gate
SP=<scratch> bash tower/build-tower.sh --dev    # fast loop: start from the committed seed binaries
```

`$SP` is a scratch directory (it may be empty or not exist yet). `lib/scratch.sh` mints the two seed binaries from
in-repo source, so no host compiler or linker enters the produced chain.

## Layout

- `L0/`: the hand-hex seed, then hex1, hex2 and the M0 assembler (the root of trust); `chain-proof.sh`
- `L1/`: `c-to-rzir.m0`, the C front-end written in M0
- `L2/`: `ccc.c`, the self-hosting C compiler, and the C backend stages
- `L3/`: `rzrs.c`, the Rust-subset compiler written in C
- `L4/`: the first Rust compiler written in Rust, and `bootstrap.sh`
- `L5/`: the self-hosting Rust bootstrap, `cf.hoist.seed` and `selfhost.sh`
- `L6/`: the next Rust bootstrap tier and `stage.sh`
- `sem0/`: SEM0 (`stages/S1-lex-parse-ast/s1_parser.rs`), its 28 hops and its stage gates
- `backend-cf/`: `cf.rs`, the backend's code generator
- `backend-toolchain/`: `rzir-{ld,opt,ra}.rs` and `provenance.sh`, the trust gate
- `backend-live/`: builds the all-Rust backend
- `backend-llvm/`: the LLVM build of SEM0, admitted only when it matches the hex-rooted build byte for byte
- `parity/`, `corpus/`, `census/`: the parity, corpus and whole-crate gates against rustc
- `ourc/`, `semc/`, `driver/`, `tcc/`, `rust-closure/`, `vendor/`: ourc's gate, helpers and test sets
- `lib/scratch.sh`, `build-tower.sh`, `promote.sh`, `gates.md`

## Trust

`backend-toolchain/provenance.sh` is the trust gate. It re-derives the whole chain from the hand-hex inside the gate
and checks it by diverse double compilation (Wheeler). `--gates` ends on it. `gates.md` describes every gate.
