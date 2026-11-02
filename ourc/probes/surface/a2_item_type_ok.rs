const N: usize = 3;
struct A<const K: usize>([u8; K]);
const fn two() -> usize { 2 }
fn main() { let a: A<2> = A([1, 2]); println!("{} {}", N + a.0.len(), two()); }
