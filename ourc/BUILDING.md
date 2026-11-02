# Building ourc from the hand-written hex

```
bash ourc/release.sh OUT                                                   # the shell build
cargo run --release --manifest-path ourc/bootstrap/Cargo.toml -- --repo . OUT    # the Rust driver
```

Either one builds every compiler in the tower from `tower/L0/hex0/hex0.hand.hex`, checks each artifact against its
pinned sha256 in [`bootstrap/pins.txt`](bootstrap/pins.txt), and ends with ourc compiling itself to a byte-identical
fixed point.

## Requirements

- x86-64 Linux
- at least 24 GB of RAM: ourc compiling itself peaks near 20 GB
- `llc` and `opt` from LLVM 16 or newer, and `cc`, on `PATH`

On a 32-core machine a build takes about 20 minutes; on 4 cores, over an hour.

## The toolchain and the hash

The release hash, `9d5fde72d8e5a4929bd929c2165b339c2adf054ab8d80672f0252a2d74074078`, was produced with LLVM 22.1.8, GCC 16.2.1, binutils 2.47 and glibc 2.44. Five of the 60 pinned artifacts pass
through those host tools, so a different toolchain builds a different ourc hash, reported as host drift. The 55
tower artifacts, the LLVM-against-hex check and the self-compile fixed point must still match exactly. Each build
records its toolchain in `PROVENANCE`.

## A measured build

One full `release.sh` run from the hand-written hex, measured with `ourc/measure-build.sh` (a systemd scope, sampled
once a second, plus the kernel's own `memory.peak` for the scope):

| | |
|---|---|
| machine | AMD Ryzen AI Max+ 395, 16 cores / 32 threads, 128 GB RAM |
| system | CachyOS, Linux 7.2.7; LLVM 22.1.8, GCC 16.2.1, binutils 2.47, glibc 2.44 |
| wall time | 19 min 51 s |
| peak memory | **20.8 GB** for the whole build (`memory.peak`); 20.4 GB of it program memory |
| largest process | ourc compiling itself (`ourc.g1`): 19.9 GB |

Where the time and memory go:

| stage | from | peak memory |
|---|---|---|
| the tower: hex0 up to L6, then SEM0's 28 hops and fixed points | 0 s | 1.8 GB |
| SEM0 lowered to LLVM, checked against the hex-built SEM0 | 8 min 11 s | 6.7 GB |
| SEM0 builds ourc (`ourc.g0`) | 14 min 37 s | 4.7 GB |
| ourc compiles itself three times (`ourc.g0`, `ourc.g1`, `ourc.g2`) | 19 min 10 s | 20.4 GB |


## Measuring a build

`bash ourc/measure-build.sh OUT` runs `release.sh` the same way and writes the samples and a summary to `OUT`. It
needs systemd (`systemd-run --user`).

## The tower's gates

`SP=<scratch> bash tower/build-tower.sh --gates` builds the tower and runs every gate, ending on the trust gate,
`tower/backend-toolchain/provenance.sh`. Three gates need the full development tree and are skipped here.
