// RUSTC-TARGET S8/PAT (P-RANGE-VALUE-IN-VARIANT-ARG): a bare range `lo..hi` used as a variant-constructor
// argument -- `Left(0..10)`. Before the fix the range under-parsed there (parse_expr stopped at `0`), and
// the leftover `..10` bled into the enclosing context; in a NON-LAST match arm it swallowed the next arm's
// `_` catch-all -> a false `non-exhaustive patterns` reject (either`s `fn iter()` test). Now emit_range_vec
// materializes the range in variant-/tuple-struct-arg position, so the arm boundary is respected. This
// probe exercises the control-flow fix (the range payload's own iteration via `if let` is a separate,
// pre-existing payload-kind-tagging gap, deliberately not tested here).
use std::ops::Range;
enum W { A(Range<i32>), B(i32) }
fn pick(x: i32) -> i32 {
    match x {
        3 => { let _w = W::A(0..10); 1 }
        7 => { let _w = W::A(5..8); 2 }
        _ => { let _w = W::B(0); 3 }
    }
}
fn main() {
    println!("{} {} {}", pick(3), pick(7), pick(99));
}
