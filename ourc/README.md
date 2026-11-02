# ourc

ourc is a Rust compiler written in Rust that compiles itself. It reads a single Rust source file and emits LLVM IR. With
`-o` it then runs `llc` and the system C compiler to produce a native executable.

**The promise of v0.1: ourc compiles a subset of Rust correctly, and refuses the rest.** A program ourc accepts runs the
way rustc's build of it runs. A program outside the subset gets an error and exit code 1, never a wrong answer. See
[LIMITATIONS.md](LIMITATIONS.md) for what the subset covers and how this was measured, and
[REFERENCE.md](REFERENCE.md) for the feature-by-feature list.

## Requirements

- x86-64 Linux
- `llc` from LLVM (any recent release), on `PATH` or named by `$LLC`
- a C compiler/linker, `cc` on `PATH` or named by `$CC` (used only to link; ourc's runtime needs libc and libm)

## Install

From the release tarball: unpack it and put `ourc` on your `PATH`. The tarball's `SHA256SUMS` lists the binary's hash.

From source: see `release.sh` (it rebuilds the release binary and checks that it reproduces the published hash).

## Usage

```
ourc prog.rs -o prog        # compile and link an executable
ourc prog.rs --emit-ir      # write LLVM IR to stdout (status and errors go to stderr)
```

Options:

| option | meaning |
|---|---|
| `-o PATH` | build an executable at `PATH` (runs `${LLC:-llc} -O2`, then `${CC:-cc}`) |
| `--emit-ir` | print the LLVM IR module on stdout instead of linking |
| `-O` | accepted for compatibility; executables are always built with `llc -O2` |
| `--time-limit N` | refuse a compile that runs longer than N seconds (default 25; 0 = no limit) |
| `--mem-limit GB` | refuse a compile that needs more than GB GiB of memory (default 8; 0 = no limit) |

A rejected program prints `ourc[O0] prog.rs -> error[lo..hi]: ...` on stderr and exits 1, and no file is written. A
compile that hits a limit, or an internal crash, also exits 1 with a one-line message saying which.

## Example

```rust
// hello.rs
struct Point { x: i32, y: i32 }

fn dist2(p: &Point) -> i32 {
    p.x * p.x + p.y * p.y
}

fn main() {
    let pts = vec![Point { x: 3, y: 4 }, Point { x: 1, y: 1 }];
    let mut total = 0;
    for p in pts.iter() {
        total += dist2(p);
    }
    println!("sum of squared distances: {}", total);
}
```

```
$ ourc hello.rs -o hello
ourc[O0] hello.rs -> ok int = <runtime> [hir=2 mir=3] ir_lines=10
$ ./hello
sum of squared distances: 27
```

## Self-hosting

`ourc` compiles its own source (`src/`, about 50,000 lines) and reaches a fixed point. ourc′ is ourc built by ourc, and
ourc″ is ourc′ built by ourc′. The SEM0-built ourc, ourc′ and ourc″ all emit byte-identical IR for ourc's own source, and
ourc′ and ourc″ are byte-identical executables. A self-build needs `--mem-limit 0 --time-limit 0`: it takes about 14 s and
peaks near 20 GB.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Reporting a wrong answer

A program that ourc accepts and that then behaves differently from rustc's build is a bug in the promise, the most
important kind of bug ourc can have. Please report it with the program, both outputs, and `ourc --emit-ir`'s first
stderr line.
