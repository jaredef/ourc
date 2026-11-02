fn adder(k: i64) -> impl Fn(i64) -> i64 { move |x| x + k } fn main() { let a = adder(40); println!("{}", a(2)); }
