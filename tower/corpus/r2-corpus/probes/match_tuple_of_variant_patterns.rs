// RUSTC-TARGET PAT (P-MATCH-TUPLE-VARIANT-PATTERN): a match on a TUPLE scrutinee whose arms use
// nested variant patterns per element -- `match (a, b) { (Left(x), Left(y)) => .., (d, s) => .. }`.
// The either crate's `clone_from` uses exactly this (variant arms + a catch-all). Before the fix, the
// tuple-pattern element parser only handled `_`/literal/plain-ident, so a variant element `Left(x)`
// read `Left` as a binding and left POS at `(x)` -- the element loop then spun forever (never
// advancing), flooding the symbol table (an infinite hang). Now each variant element disc-tests the
// element cell and binds its payload; a trailing catch-all `(d, s)` stays irrefutable.
enum Either<L, R> { Left(L), Right(R) }
use Either::{Left, Right};
fn combine(a: Either<i32, i32>, b: Either<i32, i32>) -> i32 {
    match (a, b) {
        (Left(x), Left(y)) => x + y,
        (Left(x), Right(y)) => x - y,
        (d, s) => 0,
    }
}
fn main() {
    println!("{} {} {}",
        combine(Left(2), Left(3)),
        combine(Left(9), Right(4)),
        combine(Right(5), Left(20)));
}
