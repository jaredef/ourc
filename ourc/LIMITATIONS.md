# ourc v0.1 — limitations

## The promise

**ourc compiles a subset of Rust correctly, and refuses the rest.** If ourc accepts a program, the executable it builds
produces the same output and exit code as rustc's build. If a program is outside the subset, ourc says so and exits 1.
When ourc cannot compile something faithfully, it refuses it instead of guessing; a refusal is never a wrong answer.

ourc is **under active development**, and full parity with rustc's test suite is a goal of the project. Today its
checking is still partial, so it does not yet reject every program rustc rejects (see "Accepts invalid programs"
below). Its correctness promise is about the programs it accepts.

## How it was measured

Every test in rustc 1.99.1's `tests/ui` suite (20,072 files) was run against the v0.1.1 release binary (sha256
`9d5fde72d8e5a4929bd929c2165b339c2adf054ab8d80672f0252a2d74074078`) with [ui-full.sh](ui-full.sh).

- **run-pass** tests: the test is OK if ourc accepts it and the built executable's stdout and exit code equal rustc's.
  It is WRONG if they differ.
- **check-pass / build-pass** tests: OK if ourc accepts.
- **compile-fail** tests: OK if ourc rejects.

| class | tests | OK | WRONG | refused | crash |
|---|---|---|---|---|---|
| run-pass | 3,760 | 1,103 | **0** | 2,573 | 0 |
| check/build-pass | 4,153 | 1,550 | — | 2,603 | 0 |
| compile-fail | 12,159 | 8,184 (rejected) | — | — | 0 |

10,837 of the 20,072 tests are OK. The 84 remaining run-pass tests could not be built or run by rustc itself in the test
environment and are not counted. "crash" counts both ourc crashing and a program it built crashing where rustc's does not.

## What is not supported

A refused program names the construct ourc stopped at. These are the largest areas outside the v0.1 subset, measured over the
refused valid programs:

- **Most of the standard library.** ourc models a core of `std`: `Vec`, `String`/`&str`, `Option`, `Result`, `Box`, `Rc`,
  `HashMap`, iterators and their common adapters, formatting macros, `std::fs::read_to_string`, `std::env::args`,
  `std::process::exit`. Other modules, types and methods are refused.
- **Macros beyond the common forms.** `macro_rules!` with the usual fragment kinds works; an inner repetition separator other than `,`
  and some fragment combinations are refused. Procedural macros and derive macros other than the built-in derives are not
  supported.
- **Unstable features.** Any program that needs `#![feature(..)]` is refused (ourc targets stable Rust).
- **Advanced type and trait features.** Const generics on types, higher-ranked trait bounds in some positions, trait
  objects of `std` traits, many associated-type and `where`-clause forms, and complex trait resolution.
- **Concurrency and low-level features.** `async`/`await`, coroutines, threads beyond simple forms, inline assembly,
  `union`s, and most of `unsafe` FFI (`extern "ABI"` items).
- **Layout attributes.** `#[repr(packed)]` and `#[repr(align(N))]` are not honoured, so a program that observes layout
  under them (`size_of`, `align_of`, `offset_of`, addresses) is refused.
- **Some `{:?}` output.** `Debug` formatting of a `HashMap` inside `format!`, of generic enum payloads, and of sequences
  of `()` is refused rather than printed incorrectly.
- **Multi-crate programs.** One source file plus its `mod` files; `extern crate` and Cargo dependencies are not
  supported.

## Accepts invalid programs

ourc implements only part of rustc's checking (borrow checking, trait coherence, lint-level errors, and much of type
inference are partial). It accepts some programs that rustc rejects: 3,975 of the 12,159 compile-fail tests. Closing
that gap is part of the goal of full rustc parity; until then, use rustc to check whether a program is valid Rust.

## Platform and resources

- x86-64 Linux only. Needs `llc` (LLVM 16 or newer) and a C compiler/linker (`cc`) at run time to produce executables.
- Integers and pointers are modelled as 64-bit words; `f32`, `i128` and `u128` are refused.
- By default a compile is refused after 25 seconds or 8 GiB of memory (`--time-limit`, `--mem-limit`; 0 lifts either).
  ourc compiling its own source needs both lifted.
- Memory allocated by a compiled program is not freed while it runs; long-running or allocation-heavy programs can
  run out of memory, and then exit with "memory allocation failed".
