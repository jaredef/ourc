fn make_adder(n: i64) -> impl Fn(i64) -> i64 { move |x| x + n } fn main() { let a = make_adder(10); println!("{} {}", a(1), a(2)); }
