fn parse_digit(c: i32) -> Result<i32, i32> {
    if c >= 48 && c <= 57 { Ok(c - 48) } else { Err(c) }
}
fn two_digits(a: i32, b: i32) -> Result<i32, i32> {
    let x = parse_digit(a)?;
    let y = parse_digit(b)?;
    Ok(x * 10 + y)
}
fn main() {
    match two_digits(52, 55) { Ok(n) => println!("num {}", n), Err(e) => println!("bad {}", e) }
    match two_digits(52, 88) { Ok(n) => println!("num {}", n), Err(e) => println!("bad {}", e) }
    println!("is_err {}", two_digits(65, 66).is_err() as i32);
}
