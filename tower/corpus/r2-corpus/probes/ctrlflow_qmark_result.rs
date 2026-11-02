fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n + 1)
}
fn main() { println!("{}", parse("41").unwrap()); }
