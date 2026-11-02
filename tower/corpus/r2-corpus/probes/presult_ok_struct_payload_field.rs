struct S { pos: u32 }
fn f(n: i64) -> Result<S, i64> { if n < 0 { Err(7) } else { Ok(S { pos: 42 }) } }
fn main() { match f(1) { Ok(v) => println!("{}", v.pos), Err(e) => println!("e{}", e) } }
