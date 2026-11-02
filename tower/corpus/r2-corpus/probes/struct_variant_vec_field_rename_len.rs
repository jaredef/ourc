enum E { C { name: String, args: Vec<i64> } }
fn f(e: &E) -> i64 { match e { E::C { name: _n, args } => args.len() as i64 } }
fn main() { let c = E::C { name: String::from("x"), args: vec![1, 2, 3] }; println!("{}", f(&c)); }
