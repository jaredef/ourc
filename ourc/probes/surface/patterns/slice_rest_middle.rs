fn f(s: &[i64]) -> i64 { match s { [first, mid @ .., last] => first * 100 + mid.len() as i64 * 10 + last, [x] => *x, [] => 0 } } fn main() { println!("{} {} {}", f(&[1, 2, 3, 4]), f(&[9]), f(&[])); }
