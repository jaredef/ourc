fn a(p: P) -> i32 { match p { P { x, y } => x, _ => 0 } }
fn b(t: (i32, i32)) -> i32 { match t { (a, b) => a } }
fn c(o: Option<i32>) -> i32 { match o { Some(ref x) => 1, None => 0 } }
fn d(n: i32) -> i32 { match n { 1 | 2 => 1, 0..=9 => 2, x @ 10 => x, _ => 0 } }
fn e(q: Q) -> i32 { match q { Q { a: n, .. } => n } }
fn main() {}
