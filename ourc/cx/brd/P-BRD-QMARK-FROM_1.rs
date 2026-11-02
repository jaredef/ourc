use std::num::ParseIntError;
#[derive(Debug)] enum AppErr { Parse(ParseIntError), Neg(i64) }
impl From<ParseIntError> for AppErr { fn from(e: ParseIntError) -> Self { AppErr::Parse(e) } }
fn parse_pos(s: &str) -> Result<i64, AppErr> { let n: i64 = s.trim().parse()?; if n < 0 { return Err(AppErr::Neg(n)); } Ok(n) }
fn main() { println!("{:?}", parse_pos(" 42 ")); println!("{:?}", parse_pos("-3")); println!("{}", matches!(parse_pos("x"), Err(AppErr::Parse(_)))); }
