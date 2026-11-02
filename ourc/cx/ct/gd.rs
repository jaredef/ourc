fn id<T: Copy>(x: T) -> T { x }
fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T { let mut m = xs[0]; for &x in xs { if x > m { m = x; } } m }
fn main() { let a = id(1.5); println!("{}", a); println!("{}", id('z')); let f = largest(&[1.5, 0.5]); println!("{}", f); println!("{}", largest(&['a', 'z'])); println!("{} {}", largest(&[1.5, 2.5]), largest(&[3, 9])); }
