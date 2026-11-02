// R2a P6 PUB-CRATE guard: within ONE crate `pub(crate)` is visible everywhere (only ANOTHER crate is refused: E0603 --
// p5-oracle 25-pub-crate)
mod a { pub(crate) fn inner() -> i32 { 5 } } fn main(){ println!("{}", a::inner()); }
