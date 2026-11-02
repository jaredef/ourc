// RUSTC-FE sub-arc 01 (rustc-lexer) / LEX: a let-bound Option<&T> from an iterator/slice method
// (find/get/max/first) must keep its ref-payload tag so `*r.unwrap()` / `if let Some(x)=r` is a no-op.
// Was lost at the let-binding (SOPTREF added) -> the bound Option<&T> segfaulted on deref (INLINE worked).
// rustc parsers bind `let tok = iter.find(..)` pervasively.
fn main() {
    let v: Vec<i32> = vec![10, 20, 30];
    let a = v.get(1); println!("{}", *a.unwrap());
    let b = v.iter().find(|&&x| x > 15); println!("{}", *b.unwrap());
    let c = v.iter().max(); println!("{}", *c.unwrap());
    let d = v.first(); if let Some(x) = d { println!("{}", *x); }
}
