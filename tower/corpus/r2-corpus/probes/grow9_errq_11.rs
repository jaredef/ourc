enum MyErr { DivZero, Neg }
fn checked_div(a: i64, b: i64) -> Result<i64, MyErr> {
    if b == 0 { return Err(MyErr::DivZero); }
    if a < 0 { return Err(MyErr::Neg); }
    Ok(a / b)
}
fn compute(a: i64, b: i64, c: i64) -> Result<i64, MyErr> {
    let x = checked_div(a, b)?;
    let y = checked_div(x, c)?;
    Ok(y + 1)
}
fn main() {
    let r = match compute(100, 5, 2) { Ok(v) => v, Err(_) => -1 };
    let e = match compute(100, 0, 2) { Ok(v) => v, Err(_) => -99 };
    println!("{} {}", r, e);
}
