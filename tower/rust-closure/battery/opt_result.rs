fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}
fn checked_div(a: i32, b: i32) -> Result<i32, i32> {
    if b == 0 { Err(1) } else { Ok(a / b) }
}
fn combine(a: i32, b: i32) -> Option<i32> {
    let x = half(a)?;
    let y = half(b)?;
    Some(x + y)
}
fn main() {
    match half(10) { Some(v) => println!("half {}", v), None => println!("odd") }
    match half(7) { Some(v) => println!("half {}", v), None => println!("odd") }
    println!("unwrap {}", half(8).unwrap());
    println!("unwrap_or {}", half(3).unwrap_or(-1));
    println!("is_some {}", half(4).is_some() as i32);
    match checked_div(20, 4) { Ok(q) => println!("ok {}", q), Err(e) => println!("err {}", e) }
    match checked_div(5, 0) { Ok(q) => println!("ok {}", q), Err(e) => println!("err {}", e) }
    println!("res unwrap {}", checked_div(9, 3).unwrap());
    match combine(10, 8) { Some(v) => println!("combine {}", v), None => println!("none") }
    match combine(10, 7) { Some(v) => println!("combine {}", v), None => println!("none") }
}
