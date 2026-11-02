// RUSTC-TARGET S1 (P-CLOSURE-PARAM-WHERE-FN-BOUND): a closure-typed generic param whose Fn/FnOnce/FnMut
// bound is in a `where` clause (not inline `<F: Fn(..)>`). Calling it (`f(x)`) must dispatch as a closure
// call, not reject as an undefined free fn. Pervasive: the either crate's `map_left`/`map_right`/
// `map_either` all use `where F: FnOnce(L) -> M`. rzrs only scanned inline `<..>` bounds; the where-clause
// (which follows the params in source) was skipped, so `f: F` was untagged -> `f(x)` hit the E0425 gate.
// Fixed by pre-scanning the where clause for `Ident: Fn..` bounds before the param loop.
fn apply<F, M>(f: F, x: i32) -> M where F: FnOnce(i32) -> M { f(x) }
fn twice<F, G>(f: F, g: G, x: i32) -> i32 where F: Fn(i32) -> i32, G: Fn(i32) -> i32 { f(x) + g(x) }
fn main() {
    println!("{} {}", apply(|n| n * 3, 4), twice(|n| n + 1, |n| n * 2, 5));
}
