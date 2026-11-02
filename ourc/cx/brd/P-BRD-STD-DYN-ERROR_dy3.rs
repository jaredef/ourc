// WITNESS P-STD-ERROR: std::error::Error impls, Box<dyn Error>, From conversions via ?, source chains
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
#[derive(Debug)]
enum AppErr { Parse(ParseIntError), Range(i64), Io(String) }
impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { AppErr::Parse(e) => write!(f, "parse: {}", e), AppErr::Range(n) => write!(f, "out of range: {}", n), AppErr::Io(s) => write!(f, "io: {}", s) }
    }
}
impl Error for AppErr { fn source(&self) -> Option<&(dyn Error + 'static)> { match self { AppErr::Parse(e) => Some(e), _ => None } } }
impl From<ParseIntError> for AppErr { fn from(e: ParseIntError) -> Self { AppErr::Parse(e) } }
fn check(s: &str) -> Result<i64, AppErr> { let n: i64 = s.parse()?; if n > 100 { Err(AppErr::Range(n)) } else { Ok(n) } }
fn dynamic(s: &str) -> Result<i64, Box<dyn Error>> {
    if s.is_empty() { return Err("empty input".into()); }
    let n = check(s)?;
    let f: f64 = "1.5".parse()?;
    Ok(n + f as i64)
}
fn main() { match dynamic("abc") { Ok(n) => println!("ok {}", n), Err(e) => println!("err {} {}", e, e.downcast_ref::<AppErr>().is_some()) } }
