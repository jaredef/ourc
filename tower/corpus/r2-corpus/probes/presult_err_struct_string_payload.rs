struct E { msg: String, at: u32 }
fn p(n: i64) -> Result<i64, E> { if n < 0 { return Err(E { msg: String::from("neg"), at: 0 }); } Ok(n) }
fn main() { match p(-1) { Ok(v) => println!("ok {}", v), Err(e) => println!("{} {}", e.msg, e.at) } }
