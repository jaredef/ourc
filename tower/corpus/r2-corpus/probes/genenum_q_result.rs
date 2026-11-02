fn parse(x: i32) -> Result<i32, i32> {
    if x < 0 { Err(x) } else { Ok(x) }
}
fn run() -> Result<i32, i32> {
    let a = parse(5)?;
    let b = parse(6)?;
    Ok(a + b)
}
fn main() {
    println!("{}", run().unwrap());
}
