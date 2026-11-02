fn f(s: &[i64]) -> i64 { match s { [] => 0, [x] => *x, [x, rest @ ..] => *x + f(rest) } } fn main() { println!("{}", f(&[1, 2, 3])); }
