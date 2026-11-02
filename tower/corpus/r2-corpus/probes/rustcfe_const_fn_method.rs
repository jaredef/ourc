// RUSTC-FE sub-arc 00 (fail-closed) / FC: a `const fn` METHOD (with a &self receiver) inside an impl
// must not hang the parser (prepass_constfns spun on the &self receiver). Real APIs use const fn methods
// pervasively (Option::is_some etc.).
struct X { v: i32 }
impl X { const fn get(&self) -> i32 { 42 } pub const fn dbl(&self) -> i32 { self.v * 2 } }
fn main() { let x = X { v: 21 }; println!("{} {}", x.get(), x.dbl()); }
