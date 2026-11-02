// P-BRD-GENERIC-INTO / USER-TRYFROM / TRY-INTO / STD-NUMCONV / QMARK-FROM / STD-INTO / PARSEERR-DISPLAY: the
// From/Into/TryFrom/TryInto surface (user + std), `?` converting a parse error through From, and ParseIntError Display.
use std::convert::{TryFrom, TryInto};
use std::num::ParseIntError;
#[derive(Debug)] struct Meters(f64);
impl From<u32> for Meters { fn from(x: u32) -> Self { Meters(x as f64) } }
#[derive(Debug)] struct Even(u32);
impl TryFrom<u32> for Even { type Error = String; fn try_from(x: u32) -> Result<Self, String> { if x % 2 == 0 { Ok(Even(x)) } else { Err(format!("{} odd", x)) } } }
#[derive(Debug)] enum AppErr { Parse(ParseIntError), Neg(i64) }
impl From<ParseIntError> for AppErr { fn from(e: ParseIntError) -> Self { AppErr::Parse(e) } }
fn parse_pos(s: &str) -> Result<i64, AppErr> { let n: i64 = s.trim().parse()?; if n < 0 { return Err(AppErr::Neg(n)); } Ok(n) }
fn to_m<T: Into<Meters>>(x: T) -> f64 { x.into().0 }
fn main() {
    let r: Result<Even, _> = 10u32.try_into();
    println!("{} {:?} {:?} {:?}", to_m(7u32), Even::try_from(5), r, u8::try_from(300i32).is_err());
    println!("{:?} {:?} {}", parse_pos("42"), parse_pos("-3"), matches!(parse_pos("x"), Err(AppErr::Parse(_))));
    let s: String = 'c'.into(); let v: Vec<u8> = "hi".into();
    println!("{} {:?} {}", s, v, f64::from(2.5f32));
    let e = "x".parse::<i64>().unwrap_err();
    println!("{}", format!("[{}]", e));
}
