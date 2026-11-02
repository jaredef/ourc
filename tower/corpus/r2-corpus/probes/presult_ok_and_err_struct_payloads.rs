struct Ok_ { v: i64 }
struct E { msg: String }
fn f(n: i64) -> Result<Ok_, E> { if n < 0 { Err(E { msg: String::from("bad") }) } else { Ok(Ok_ { v: n * 2 }) } }
fn main() {
    match f(3) { Ok(o) => println!("ok {}", o.v), Err(e) => println!("err {}", e.msg) }
    match f(-1) { Ok(o) => println!("ok {}", o.v), Err(e) => println!("err {}", e.msg) }
}
