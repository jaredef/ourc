struct E { at: u32 }
fn p(n: i64) -> Result<i64, E> { if n < 0 { return Err(E { at: 7 }); } Ok(n) }
fn main() { match p(-1) { Ok(v) => println!("ok {}", v), Err(e) => println!("err {}", e.at) } }
