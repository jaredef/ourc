fn f(o: Option<i64>) -> i64 { let Some(x) = o else { return 0; }; x * 2 } fn main() { println!("{} {}", f(Some(21)), f(None)); }
