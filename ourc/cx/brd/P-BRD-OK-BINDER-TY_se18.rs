use std::num::ParseIntError;
#[derive(Debug)] enum AppErr { Parse(ParseIntError), Range(i64) }
impl From<ParseIntError> for AppErr { fn from(e: ParseIntError) -> Self { AppErr::Parse(e) } }
use std::fmt;
impl fmt::Display for AppErr { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { AppErr::Parse(e) => write!(f, "parse: {}", e), AppErr::Range(n) => write!(f, "r {}", n) } } }
fn check(s: &str) -> Result<i64, AppErr> { let n: i64 = s.parse()?; if n > 100 { Err(AppErr::Range(n)) } else { Ok(n) } }
fn main() { match check("42") { Ok(n) => println!("ok {}", n), Err(_) => println!("err") } }
