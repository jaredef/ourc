fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T { let mut m = xs[0]; for &x in xs { if x > m { m = x; } } m }
fn main() { println!("{} {} {:.1} {}", largest(&[3, 9, 4]), largest(&[1.5, 0.5]), largest(&[1.5, 0.5]), largest(&['a', 'z'])); }
