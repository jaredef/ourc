struct Err { pos: u32 }
fn parse(n: i64) -> Result<i64, Err> {
    if n < 0 { return Err(Err { pos: 7 }); }
    if n == 0 { Ok(1) } else {
        let inner = parse(n - 1)?;
        Ok(inner + n)
    }
}
fn main() {
    match parse(3) {
        Ok(v) => { println!("ok {}", v); }
        Err(e) => { println!("err {}", e.pos); }
    }
}
