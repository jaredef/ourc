struct S { x: i32 }
fn take(s: S) -> i32 { s.x }
fn main() { let a = S { x: 5 }; let _b = take(a); std::process::exit(take(a)); }
