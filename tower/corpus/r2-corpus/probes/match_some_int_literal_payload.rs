fn g(o: Option<i32>) -> i64 { match o { Some(5) => 1, Some(_) => 2, None => 0 } }
fn main() { println!("{}", g(Some(5))); }
