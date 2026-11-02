enum E { A(i32), B(String, i32), C }
fn f() -> E { E::A(5) }
fn g(s: String) -> E { E::B(s, 1) }
fn main() {}
