fn f(o: Option<i32>) -> i32 { if let Some(x) = o && x > 0 { x } else { 0 } }
fn main() {}
