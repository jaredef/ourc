fn parse(ok: bool) -> Result<i64, i64> {
    if ok { Ok(21) } else { Err(9) }
}
fn compute(ok: bool) -> Result<i64, i64> {
    let a = parse(ok)?;
    Ok(a * 2)
}
fn main() {
    let r = match compute(true) {
        Ok(v) => v,
        Err(e) => e,
    };
    let s = match compute(false) {
        Ok(v) => v,
        Err(e) => e,
    };
    println!("{}", r + s);
}
