fn f(s: &[i64]) -> i64 { if s.len() == 0 { 0 } else { s[0] + f(&s[1..]) } } fn main() { println!("{}", f(&[1, 2, 3])); }
