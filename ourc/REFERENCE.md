# ourc v0.1 — language and library reference

What ourc compiles, item by item. [LIMITATIONS.md](LIMITATIONS.md) states the promise and the headline numbers; this
page is the detail behind them.

**Status words.** *Supported*: ourc compiles it and the program behaves as rustc's build does. *Partial*: some forms
compile, the rest are refused; the row says which. *Refused*: ourc stops with an error and exits 1. A refusal is
never a wrong answer, so anything not listed here is either refused or (rarely) a bug — see [Known
issues](#known-issues).

**Evidence.** Each row rests on one or more of: the compiler source (`src/`), the boundary probes in
`probes/surface/` (pairs of `*_ok.rs` / `*_refused.rs`, each checked against rustc by `surface.sh`), the end-to-end
programs in `progs/`, and the run of rustc 1.99.1's `tests/ui` suite (see [By test area](#by-test-area)).
Rows marked † were checked directly against the release binary.

## Programs and the driver

- One source file plus the files of its `mod` declarations. Edition 2021 semantics.
- `fn main()` and `fn main() -> Result<(), E>` are supported; other `main` signatures are refused.
- Integer arithmetic follows rustc's **release** profile: overflow wraps, it does not panic.
- Panics (`panic!`, index out of bounds, `unwrap` on `None`, divide by zero) exit with code 101, like rustc.†
- Options, limits and output are documented in [README.md](README.md).

## Items

| item | status | notes |
|---|---|---|
| `fn` (free, nested, `const fn`, `unsafe fn`, `extern "C" fn` with a body) | supported | |
| `struct` (named, tuple, unit), struct update `..base` | supported | |
| `enum` (payloads, struct variants, explicit discriminants) | supported | `#[non_exhaustive]` within the crate |
| `const`, `static`, `static mut` | partial | refused: a `const` whose type implements `Drop`; a non-`Sync` `static` |
| `trait` (default methods, associated types and consts, supertraits, generic methods) | supported | |
| `impl` (inherent, trait, generic, blanket) | supported | orphan rule enforced as rustc does |
| `mod`, `use` (`self`/`super`/`crate`, trees, `as`, globs, `pub(crate)`) | supported | inline modules and module files |
| `type` aliases, including generic ones | supported | |
| `union` | refused | |
| `extern "C" { … }` blocks (FFI) | refused | |
| `extern crate`, Cargo dependencies, multi-crate programs | refused | |
| `async fn`, `await`, coroutines | refused | |

## Generics, traits and lifetimes

| feature | status | notes |
|---|---|---|
| type parameters, bounds, `where`, turbofish, defaults, `?Sized` | supported | |
| associated-type equality bounds (`T: Iterator<Item = u8>`) | supported | |
| lifetimes and elision | supported | lifetimes are checked only partly (see LIMITATIONS, "Accepts invalid programs") |
| higher-ranked bounds `for<'a>` | supported† | in `where`, `dyn`, `impl` and fn-pointer positions |
| const generics | partial | on functions only; on types and traits refused |
| `dyn Trait` (`Box<dyn T>`, `&dyn T`, `dyn Fn`) | partial | user traits; trait objects of std traits refused; non-dyn-compatible traits refused |
| `impl Trait` | partial | argument and return position; elsewhere refused (E0562) |
| operator overloading | partial | `Add`, `Sub`, `Mul`, `Div`, `PartialEq` and the compound-assignment traits; `Neg` and `Index` on user types refused† |
| specialization, `type_alias_impl_trait`, other nightly features | refused | any `#![feature(..)]` is refused |

## Expressions and patterns

| feature | status | notes |
|---|---|---|
| closures (`Fn`/`FnMut`/`FnOnce`, `move`, boxed, returned as `impl Fn`, coerced to fn pointers) | supported | |
| `match`: guards, `\|`, ranges, `@`, slice patterns, nested enums | partial | refused: `..` before the end of a tuple pattern; nested patterns in struct-variant fields; writes through `ref mut`; non-primitive consts as patterns |
| `if let`, `while let`, `let … else` | supported | |
| `if let` chains (`if let P = e && c`) | refused | an edition-2024 feature |
| `loop`/`while`/`for`, labels, `break` with a value, labelled blocks | supported | |
| ranges (`a..b`, `..=`, open, as values, in slicing) | supported | |
| `?` on `Option` and `Result` | supported | |
| `as` casts | partial | numeric casts (float→int saturates, as rustc); of the char casts only `u8 as char` |
| destructuring assignment | partial | tuples and arrays of plain names |
| `&mut` | partial | `&mut` parameters and places; some mutating calls in value position refused |
| `unsafe` blocks, raw pointers, `ptr::null`/`eq`, `transmute`, `size_of` | partial | a pointer's address as an integer is refused |
| inline assembly (`asm!`) | refused | |

## Types

| type | status | notes |
|---|---|---|
| `i8`–`i64`, `u8`–`u64`, `isize`, `usize`, `bool`, `char` | supported | |
| `i128`, `u128` | refused | |
| `f64` | supported | |
| `f32` | refused† | "f32 is not implemented; f64 is" |
| references, slices, arrays (`[x; N]`, nested), tuples | supported | |
| `str` / `String` | supported | UTF-8 text in source and at run time† |
| fn pointers (including `extern "C" fn`, `unsafe fn`) | supported | |
| `!` | partial | as a return type |

## Attributes

| attribute | status | notes |
|---|---|---|
| `#[derive(..)]` | partial | `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`; any other derive refused |
| `#[repr(C)]`, `#[repr(u8)]` and other integer reprs | supported | |
| `#[repr(packed)]`, `#[repr(align(N))]` | partial | not honoured; programs that observe the layout are refused |
| `#[cfg(..)]` on items | supported | |
| `#![feature(..)]` | refused | |
| procedural macros | refused | |

## `macro_rules!`

| feature | status | notes |
|---|---|---|
| fragments `expr`, `tt`, `ident`, `ty`, `literal`, `block`, `pat` | supported | |
| repetition with separators, `$(,)?`, tt-munching, recursion | supported | |
| hygiene of `let` bindings and labels | supported | |
| `:meta` fragment, attributes inside matchers | refused | |
| repetition separated by `;`, nested repetition with a `,` separator (`$( $a:ident = $($b:expr),* );*`) | supported† | an inner separator other than `,` is refused |
| `#[macro_export]` macros reached by a module path | refused | |

## Drop order

Supported, and among the most heavily probed areas: reverse declaration order, inner scopes first, explicit `drop`,
`let _ =`, temporaries (including the `for` head and lifetime extension), moves into iterators, reassignment, and drop
glue for enum payloads and generic types.

## Standard library

ourc does not compile `std`. It knows a core of it directly and maps those names onto its own runtime. Any std type,
method or function not listed below is refused.

### Primitive methods

- **Integers:** `pow`, `abs`, `min`, `max`, `cmp`, `count_ones`, `leading_zeros`, `trailing_zeros`, `rem_euclid`,
  `div_euclid`, `signum`, `checked_div`, `wrapping_add`/`sub`/`mul`/`neg`/`shl`/`shr`, `to_le`; `MIN` and `MAX`.
  `checked_add`/`sub`/`mul` and `saturating_add`/`sub` below 64 bits; `saturating_mul` at 16 bits or fewer.
- **`f64`:** `floor`, `ceil`, `round`, `abs`, `sqrt`, `powi`, `powf`, `max`, `min`, `signum`, `is_nan`,
  `is_infinite`, `is_finite`, `ln`, `exp`, `exp2`, `sin`, `cos`, `log2`, `log10`.
- **`char`:** `is_alphabetic`, `is_numeric`, `is_alphanumeric`, `is_whitespace`, `is_lowercase`, `is_uppercase`, the
  `is_ascii_*` forms, `to_ascii_uppercase`, `to_ascii_lowercase`, `to_string`; `char::from_u32`.

### `str` and `String`

`len`, `is_empty`, `push`, `push_str`, `chars`, `char_indices`, `bytes`, `as_bytes`, `as_str`, `to_string`,
`to_owned`, `clone`, `into`, `split`, `split_whitespace`, `trim`, `trim_start`, `trim_end`, `trim_matches`,
`trim_start_matches`, `trim_end_matches`, `contains`, `find`, `rfind`, `starts_with`, `ends_with`, `to_uppercase`,
`to_lowercase`, `repeat`, `replace`, `replacen`, `parse`; `String::new`, `with_capacity`, `from`; `join` and `concat`
on slices of strings. A closure as a string pattern is refused.

### `Vec`, arrays and slices

`len`, `is_empty`, `push`, `pop`, `get`, `insert`, `remove`, `swap_remove`, `contains`, `clear`, `truncate`, `append`,
`extend`, `reverse`, `swap`, `dedup`, `retain`, `sort`, `sort_unstable`, `sort_by`, `sort_by_key`, `binary_search`
(integer elements), `windows`, `chunks`, `to_vec`, `clone`, `iter`, `iter_mut`, `into_iter`, `as_slice`,
`as_mut_slice`; `Vec::new`, `Vec::with_capacity`, `vec![..]`. Arrays: `len`, `iter`, `into_iter`. Sorting, `max`
and `min` over struct or enum elements are refused.

### `Option` and `Result`

- **`Option`:** `unwrap`, `expect`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, `is_some`, `is_none`, `map`,
  `map_or`, `and_then`, `ok_or`, `filter`, `copied`, `cloned`.
- **`Result`:** `unwrap`, `expect`, `unwrap_or`, `is_ok`, `is_err`, `ok`, `map`, `map_err`. `and_then` and
  `unwrap_or_else` are refused.†

### Collections and smart pointers

| type | status | notes |
|---|---|---|
| `HashMap` | partial | `new`, `insert`, `get`, `remove`, `contains_key`, `len`, `is_empty`, `keys`, `iter`, `m[&k]`, `entry(k).or_insert(v)`; keys of string or integer type; `collect()` into a map refused |
| `HashSet`, `BTreeMap`, `BTreeSet`, `VecDeque` | refused† | |
| `Box` | supported | |
| `Rc`, `Arc` | partial | `new`, `clone`, `Rc::strong_count` |
| `Cell`, `RefCell`, `Mutex`, `RwLock` | partial | `new`, `get`, `set`, `borrow`, `borrow_mut`, `lock`, `write`, `into_inner` |

### Iterators

- **Adapters:** `map`, `filter`, `enumerate`, `zip`, `chain`, `flat_map`, `rev`, `take`, `skip`, `step_by`,
  `take_while`, `skip_while`, `cloned`, `copied`.
- **Consumers:** `collect` (into `Vec` or `String`), `sum`, `product`, `count`, `max`, `min`, `max_by_key`,
  `min_by_key`, `last`, `next`, `nth`, `fold`, `reduce`, `any`, `all`, `position`, `find`, `partition`.
- **Sources:** `iter::empty`, `iter::once`, `iter::repeat(..).take(n)`; user `impl Iterator`.
- An adapter whose closure prints is refused: ourc evaluates iterators eagerly, which would change the output order.

### Traits

`Clone`, `Copy`, `Debug`, `Display`, `Default` (not on enums), `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`,
`Iterator`, `IntoIterator`, `Drop`, `Deref`, `DerefMut`, `FromStr`, `Add`/`Sub`/`Mul`/`Div` and their `*Assign`
forms. A hand-written `impl Debug` used through `{:?}`, and `Display` or `to_string` of a struct, are refused.

### Macros and formatting

- **Macros:** `println!`, `print!`, `eprintln!`, `eprint!`, `format!`, `write!`, `writeln!`, `vec!`, `panic!`,
  `assert!`, `assert_eq!`, `assert_ne!`, `debug_assert!`, `unreachable!`, `todo!`, `unimplemented!`, `matches!`,
  `concat!`, `stringify!`. `dbg!` is refused.†
- **Format specs:** `{}`, `{:?}`, `{N}`, `{name}`, `{{`/`}}`, fill and alignment (`<`, `^`, `>`), `0`, width,
  precision (`{:.3}`, `{:8.3}`), `{:e}`. Refused: `{:x}`, `{:X}`, `{:b}`, `{:o}`, `{:+}`† and width from an argument
  (`{:1$}`). `{:?}` of a `HashMap` inside `format!`, of generic enum payloads and of sequences of `()` is refused.

### Free functions

| path | functions |
|---|---|
| `std::mem` | `swap`, `replace`, `take`, `size_of`, `size_of_val`, `align_of`, `transmute`, `zeroed` |
| `std::cmp` | `max`, `min`; `Ordering` |
| `std::process` | `exit` |
| `std::fs` | `read_to_string`, `read` |
| `std::env` | `args` |
| `std::thread` | `spawn` and `join`, for a thread whose closure cannot panic |
| `std::ptr` | `null`, `null_mut`, `eq`, `addr_eq`, `from_ref`, `from_mut` |

## Known issues

1. **Nothing is written to stderr on a panic or an `Err` from `main`.**† stdout and the exit code (101, 1) match
   rustc; the panic message and the `Error: …` line are missing.
2. **An out-of-date error message.** A refused format spec reports "only `{}` is implemented", though much more is.

## By test area

How ourc did on the valid programs (run-pass, check-pass and build-pass) of each directory of rustc 1.99.1's
`tests/ui`, for every directory with at least 30 of them. *OK* means ourc accepted the program and, for
run-pass tests, the executable's stdout and exit code equalled rustc's; the rest were refused. Of all 7,913 valid
programs, 2,653 are OK and none is a wrong answer.

| area | valid programs | OK | |
|---|---|---|---|
| `traits` | 640 | 224 | 35% |
| `consts` | 376 | 139 | 36% |
| `lint` | 328 | 174 | 53% |
| `const-generics` | 291 | 31 | 10% |
| `async-await` | 221 | 24 | 10% |
| `impl-trait` | 221 | 101 | 45% |
| `macros` | 205 | 61 | 29% |
| `associated-types` | 168 | 82 | 48% |
| `closures` | 147 | 61 | 41% |
| `mir` | 143 | 41 | 28% |
| `rfcs` | 133 | 29 | 21% |
| `proc-macro` | 132 | 28 | 21% |
| `imports` | 127 | 53 | 41% |
| `type-alias-impl-trait` | 125 | 6 | 4% |
| `parser` | 110 | 63 | 57% |
| `regions` | 100 | 71 | 71% |
| `nll` | 99 | 43 | 43% |
| `binding` | 98 | 59 | 60% |
| `borrowck` | 98 | 51 | 52% |
| `pattern` | 92 | 24 | 26% |
| `drop` | 90 | 26 | 28% |
| `self` | 82 | 40 | 48% |
| `codegen` | 81 | 24 | 29% |
| `coroutine` | 80 | 1 | 1% |
| `coherence` | 76 | 32 | 42% |
| `abi` | 68 | 6 | 8% |
| `higher-ranked` | 68 | 36 | 52% |
| `structs-enums` | 67 | 28 | 41% |
| `threads-sendsync` | 65 | 13 | 20% |
| `cross-crate` | 63 | 0 | 0% |
| `lifetimes` | 60 | 27 | 45% |
| `array-slice-vec` | 59 | 26 | 44% |
| `generic-associated-types` | 58 | 33 | 56% |
| `derives` | 57 | 24 | 42% |
| `unboxed-closures` | 56 | 19 | 33% |
| `coercion` | 55 | 24 | 43% |
| `associated-type-bounds` | 55 | 22 | 40% |
| `attributes` | 54 | 25 | 46% |
| `for-loop-while` | 53 | 39 | 73% |
| `resolve` | 53 | 28 | 52% |
| `specialization` | 51 | 1 | 1% |
| `match` | 50 | 28 | 56% |
| `hygiene` | 49 | 6 | 12% |
| `asm` | 49 | 0 | 0% |
| `generics` | 46 | 27 | 58% |
| `numbers-arithmetic` | 44 | 20 | 45% |
| `delegation` | 43 | 0 | 0% |
| `associated-consts` | 41 | 20 | 48% |
| `structs` | 41 | 19 | 46% |
| `iterators` | 40 | 6 | 15% |
| `simd` | 40 | 1 | 2% |
| `check-cfg` | 39 | 29 | 74% |
| `extern` | 38 | 18 | 47% |
| `transmutability` | 37 | 1 | 2% |
| `privacy` | 37 | 14 | 37% |
| `process` | 36 | 1 | 2% |
| `cfg` | 36 | 19 | 52% |
| `functions-closures` | 35 | 19 | 54% |
| `methods` | 34 | 13 | 38% |
| `sanitizer` | 33 | 3 | 9% |
| `enum` | 33 | 20 | 60% |
| `modules` | 33 | 14 | 42% |
| `box` | 32 | 10 | 31% |
| `cast` | 31 | 4 | 12% |
| `target-feature` | 31 | 3 | 9% |
| `inference` | 30 | 14 | 46% |
| `statics` | 30 | 10 | 33% |
