// R2a P5 guard: `pub const fn` / `pub unsafe fn` in a module are PUBLIC (the qualifier between `pub` and `fn` dropped the `pub`:
// "a public function (this one is private to its module)", E0603 on a public fn); a private const fn is still refused
mod l { pub const fn sq(x: usize) -> usize { x * x } } const N: usize = l::sq(3); mod u { pub unsafe fn f() -> i32 { 4 } }
fn main() { println!("{} {}", [1u8; N].len(), unsafe { u::f() }); }
