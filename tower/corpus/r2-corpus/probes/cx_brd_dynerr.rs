// P-BRD-DYN-ERROR-DISPLAY / OK-BINDER-TY / FLOAT-PARSE-Q / BOX-AUTODEREF: {} of a Box<dyn Error> (let, Err(Box::new),
// ? into Box<dyn Error>), an Ok binder of Result<i64, E> where E has Display, `let f: f64 = s.parse()?`, and builtin
// collection methods / iteration / Debug through a Box<Vec>.
use std::error::Error;
use std::fmt;
#[derive(Debug)] struct E(i32);
impl fmt::Display for E { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "e{}", self.0) } }
impl Error for E {}
fn inner(n: i32) -> Result<i32, E> { if n > 0 { Ok(n * 2) } else { Err(E(n)) } }
fn outer(n: i32) -> Result<i32, Box<dyn Error>> { let x = inner(n)?; if x > 10 { return Err(Box::new(E(99))); } Ok(x) }
fn fl(s: &str) -> Result<i64, std::num::ParseFloatError> { let f: f64 = s.parse()?; Ok(f as i64 + 1) }
fn main() {
    let b: Box<dyn Error> = Box::new(E(3));
    println!("{} {} {}", b, outer(-1).unwrap_err(), outer(6).unwrap_err());
    match inner(4) { Ok(n) => println!("ok {}", n), Err(e) => println!("err {}", e) }
    println!("{:?}", fl("2.5"));
    let mut v = Box::new(vec![1, 2]);
    v.push(3);
    let s: i32 = v.iter().sum();
    for x in v.iter() { print!("{} ", x); }
    println!("| {} {} {:?}", v.len(), s, v);
}
