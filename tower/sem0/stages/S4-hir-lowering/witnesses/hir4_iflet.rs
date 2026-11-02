fn s(o: Option<i32>) -> i32 { if let Some(x) = o { x } else { 0 } }
fn p(c: bool) -> i32 { if c { 1 } else { 2 } }
fn q(c: bool) -> i32 { let mut t = 0; if c { t = 1; } t }
fn main() {}
