enum E { R(bool, i32), N }
fn f(e: &E) -> bool { match e { E::R(m, _n) => *m, _ => false } }
fn main() {}
