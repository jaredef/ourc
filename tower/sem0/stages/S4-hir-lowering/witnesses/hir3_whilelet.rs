fn s(o: Option<i32>) -> i32 { let mut t = 0; while let Some(x) = o { t = x; } t }
fn main() {}
