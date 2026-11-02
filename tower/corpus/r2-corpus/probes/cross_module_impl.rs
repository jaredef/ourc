// RUSTC-TARGET S7 (module support): the multi-file crate shape (adler2) after external-module inlining
// -- a type in one scope, an impl (with `use super::`) in a module, cross-module method + path calls.
struct Sum { a: u32 }
impl Sum { fn base(&self) -> u32 { self.a } }
mod algo {
    use super::Sum;
    impl Sum { pub(crate) fn compute(&mut self, x: u32) { self.a += x * 2; } }
    pub fn scale(x: u32) -> u32 { x * 3 }
}
fn main() {
    let mut s = Sum { a: 10 };
    s.compute(5);
    println!("{}|{}|{}", s.base(), s.a, algo::scale(4));
}
