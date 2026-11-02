enum E { A(i32), C }
fn f() -> E { E::C }
fn h() -> i32 { std::process::exit(1) }
fn main() {}
