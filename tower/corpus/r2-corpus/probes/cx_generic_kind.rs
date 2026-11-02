// P-CX-MONO-SCALARKIND + P-CX-AGGEQ-REFMATCH witness (ourc construct-generation, 2026-10-24): a generic fn returning T
// takes T's scalar kind from its argument (f64/char print as such, not as raw words), incl. `fn f<T>(xs: &[T]) -> T`;
// a `match &v[i] { Some(prev) => prev == t }` binder over Option<Adt> holds the Adt cell (structural ==).
#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn id<X: Copy>(x: X) -> X { x }
fn largest<X: PartialOrd + Copy>(xs: &[X]) -> X { let mut m = xs[0]; for &x in xs { if x > m { m = x; } } m }

fn main() {
    println!("{} {} {} {} {}", id(1.5), id('z'), largest(&[1.5, 0.5]), largest(&['a', 'z']), largest(&[3, 9]));


}
