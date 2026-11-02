fn largest<T: PartialOrd + Copy>(v: &[T]) -> T { let mut m = v[0]; for &x in v { if x > m { m = x; } } m } fn main() { println!("{} {}", largest(&[3, 9, 2]), largest(&[1.5, 0.2])); }
