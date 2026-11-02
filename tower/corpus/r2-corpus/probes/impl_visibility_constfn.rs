// RUSTC-TARGET (S1/method-resolution, surfaced by censusing adler2): `pub fn` / `pub const fn` /
// `pub(crate) fn` / `const fn` methods in an impl block must be registered + dispatchable. The corpus
// never used method visibility, so `pub fn` silently dropped the method (the `fn` was skip_tok'd).
struct A { n: u32 }
impl A {
    pub const fn k() -> u32 { 100 }
    pub fn get(&self) -> u32 { self.n }
    pub(crate) fn dbl(&self) -> u32 { self.n * 2 }
    const fn base() -> u32 { 7 }
}
trait H { fn hash(&self) -> u64; }
impl H for A { fn hash(&self) -> u64 { u64::from(self.get()) } }   // inherent method from a trait impl (the adler2 shape)
fn main() {
    let a = A { n: 5 };
    println!("{} {} {} {} {}", a.get(), a.dbl(), a.hash(), A::k(), A::base());
}
