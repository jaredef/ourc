fn f(s: &[i64]) -> i64 { match s { [x, y] => *x + *y, _ => 9 } } fn main() { println!("{} {}", f(&[5, 6]), f(&[1, 2, 3])); }
