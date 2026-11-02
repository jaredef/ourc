enum E { C { a: i64, b: i64 } }
fn f(e: &E) -> i64 { match e { E::C { a, b } => *a + *b } }
fn main() { let c = E::C { a: 10, b: 20 }; println!("{}", f(&c)); }
