# ourc

A Rust compiler written in Rust that compiles itself. It reads a single Rust source file and emits LLVM IR; with `-o`
it runs `llc` and the system C compiler to produce a native executable.

Project site: [ourc.dev](https://ourc.dev) · source: [github.com/jaredef/ourc](https://github.com/jaredef/ourc) ·
from-hex build: [`ourc-bootstrap`](https://crates.io/crates/ourc-bootstrap)

```
cargo install ourc
ourc prog.rs -o prog        # compile and link an executable
ourc prog.rs --emit-ir      # write LLVM IR to stdout
```

Needs x86-64 Linux, plus `llc` from LLVM 16 or newer and `cc` on `PATH`.

**The promise of v0.1: ourc compiles a subset of Rust correctly, and refuses the rest.** A program ourc accepts runs the
way rustc's build of it runs; a program outside the subset gets an error and exit code 1, never a wrong answer. On
rustc 1.99.1's `tests/ui` suite it passes 10,837 of 20,072 tests, compiles 2,653 of 7,913 valid programs correctly,
and gives 0 wrong answers. See [LIMITATIONS.md](https://github.com/jaredef/ourc/blob/main/ourc/LIMITATIONS.md) and
[REFERENCE.md](https://github.com/jaredef/ourc/blob/main/ourc/REFERENCE.md).

## The goal

ourc is a trustless, hex-rooted speed run to rustc: a path from 269 hand-written bytes to a Rust compiler where every
step can be rebuilt and checked, and nothing has to be taken on faith. Its target is behavioral parity with rustc
(every program rustc accepts compiles and runs the same, every program it rejects is rejected) in a tenth of
rustc's lines of code. v0.1 is a step on that path, not the end of it.

## Two ways to get ourc

- **This crate** builds ourc with cargo in about a minute. You are trusting rustc, as with any crate.
- **[`ourc-bootstrap`](https://crates.io/crates/ourc-bootstrap)** builds ourc from 269 bytes of hand-written machine
  code, through a tower of compilers, checking every artifact against a pinned hash. It takes about 20 minutes and
  24 GB of RAM, and trusts no compiler binary.

The two agree: ourc built by rustc and ourc built from the hex emit byte-identical LLVM IR for ourc's own source.
The release binary, built from the hex, is on [GitHub](https://github.com/jaredef/ourc/releases/latest).

## License

MIT or Apache-2.0, at your option.
