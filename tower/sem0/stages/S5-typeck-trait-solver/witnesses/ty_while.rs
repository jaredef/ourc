fn f(k: i32) -> i32 { let mut i = 0; while i < k { i += 1; } i }
fn g(k: i32) -> i32 { let mut i = 0; while i < k { if i == 3 { break; } i += 1; } i }
fn main() {}
