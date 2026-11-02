# ourc-bootstrap

Builds the compiler tower and **ourc** from a hand-written hex seed, and checks every stage
against pinned hashes.

Project site: [ourc.dev](https://ourc.dev) · source: [github.com/jaredef/ourc](https://github.com/jaredef/ourc) ·
the compiler alone, built by cargo: [`ourc`](https://crates.io/crates/ourc)

```
cargo install ourc-bootstrap
ourc-bootstrap ./out          # ~20 min on 32 cores: the whole tower, ~30 SEM0 stages, then ourc compiling itself
```

The result is `out/ourc-v0.1.1-x86_64-linux/ourc` plus `SHA256SUMS`. `PROVENANCE` lists the hash of every
artifact on the way, in build order. The build needs x86-64 Linux with at least 24 GB of RAM (ourc compiling itself
peaks near 20 GB), plus `llc` and `opt` from LLVM 16 or newer and `cc` on `PATH`.

On a terminal it shows a dashboard:
- **Phases:** the six phases, with a progress bar and an ETA weighted by a reference run.
- **The current step:** what is running and for how long.
- **Artifacts:** each one as it is sealed, against its pin.
- **Checks:** the fixed points and C2, as they pass.

Press `q`, then `y`, to stop after the running step; `--resume` continues from there. `--plain`, or any
pipe, prints one line per step instead. The dashboard needs no extra crates: it is ANSI escapes and termios,
and the build itself is the same in both modes.

## The goal

ourc is a trustless, hex-rooted speed run to rustc: a path from 269 hand-written bytes to a Rust compiler where every
step can be rebuilt and checked, and nothing has to be taken on faith. Its target is behavioral parity with rustc
(every program rustc accepts compiles and runs the same, every program it rejects is rejected) in a tenth of
rustc's lines of code. v0.1 is a step on that path, not the end of it.

## What it does

1. **Packs the seeds itself.** `hex0.hand.hex` (269 bytes of machine code, written and commented by hand)
   and `derived-hex1.labeled.hex` are decoded by this program's own hex packers. The results must equal
   the committed seed binaries, and each seed must rebuild itself from its source. The seeds then build
   hex2, the M0 assembler, the M0 backend, a C compiler written in M0, and the self-hosting C compiler
   `ccc`.
2. Builds the tower's Rust bootstrap from there: a C-written Rust-subset compiler, the Rust backend
   tools, the L5 compiler and its own code generator (`cf.hoist`), and L6. The first `cf.hoist` is `cf.rs`
   compiled by L4, and it must lower L5 byte-identically to the `cf.hoist` L5 builds.
3. Climbs the SEM0 staircase. Each frozen compiler compiles the next, up to byte fixed points.
4. Builds the LLVM-backed SEM0 and admits it only if it emits SEM0 byte-identically to the hex-rooted one
   (the C2 check).
5. Builds ourc with SEM0, then has ourc compile itself twice. The IR must be identical at every step, and
   ourc.g1 and ourc.g2 must be byte-identical executables.

These are the same steps as `ourc/release.sh` and the `tower/` scripts. The test batteries those scripts
also run are left out.

## Pins

`pins.txt` holds the hash of each artifact from a reference build made by the shell scripts.

- **chain** artifacts are made by the tower alone. They must match on any machine; a mismatch stops the
  build.
- **host** artifacts pass through the host's `llc`, `opt` and `cc`. They match only with the same tool
  versions, so a mismatch is reported but doesn't stop the build.

Artifacts are named `<rung>.<what>[.<backend>][.gN|.iN]`, for example `L5.rzrs.g2`, `SEM0.h17` and
`ourc.g1`.

## What this does and doesn't prove

This program is compiled by rustc, so a compromised rustc could make it lie. It is not a new root of
trust; the hand-hex is. What it adds is a **second, independent driver** of the same build. When its
artifacts equal those of the shell build (`release.sh`), bash/coreutils and rustc would both have had to
be subverted, in agreement, to fake the result.

## Sources

The crate carries the 80 source files the build reads, in `tree/`, along with their hashes (`MANIFEST`). They are
embedded in the binary at build time. To build from a checkout instead, run
`ourc-bootstrap --repo <ourc checkout> ./out`. To refresh `tree/` before packaging, run
`ourc-bootstrap --vendor <checkout> tree`.

## License

MIT or Apache-2.0, at your option.
