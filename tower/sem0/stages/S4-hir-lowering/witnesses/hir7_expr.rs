fn a(x: i32, y: i32) -> i32 { x + y * 2 - 1 }
fn b(x: i32) -> bool { !(x < 0) && x == 1 || x != 2 }
fn c(s: S) -> i32 { s.f }
fn d(v: Vec<i32>) -> i32 { v.len() }
fn e(v: Vec<i32>) -> i32 { v[0] }
fn f() -> i32 { -5 }
fn g(p: bool) -> i32 { match p { true => 1, false => 0 } }
fn h() -> &str { "hi" }
fn main() {}
