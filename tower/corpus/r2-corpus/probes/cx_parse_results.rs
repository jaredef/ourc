// cx12-rest / cx10 witness (ourc construct-generation, 2026-10-25): str::parse Err = ParseIntError (Debug + map_err);
// Result locals with `_`/absent Err types; Result::map(|o| (..)) typing; BTreeSet iter().next(); zip(chars()).fold with a
// String accumulator; Vec<Option<_>>/Vec<Result<_,_>>/Option<Result<_,_>> Debug; a USER fn parse keeps its own Err.
use std::collections::BTreeSet;
#[derive(Debug, Default)] struct O { cfgs: Vec<(String, Option<String>)>, edition: u16 }
fn po(n: u16) -> Result<O, String> { if n > 0 { Ok(O { cfgs: vec![("feature".into(), Some("x".into()))], edition: n }) } else { Err("e".into()) } }
struct Pr { n: u32 } impl Pr { fn parse(&mut self) -> Result<u32, String> { self.n += 1; if self.n > 1 { Err("eof".to_string()) } else { Ok(self.n) } } }
fn main() {
    let r: Result<u16, _> = "20x".parse::<u16>(); println!("{:?}", r);
    match "".parse::<u16>() { Ok(v) => println!("{}", v), Err(e) => println!("{:?}", e) }
    let m = "7x".parse::<u16>().map_err(|e| format!("bad: {:?}", e)); println!("{:?}", m);
    println!("{:?}", po(2021).map(|o| (o.cfgs, o.edition)));
    let s: BTreeSet<char> = "mississippi".chars().collect(); println!("{:?} {:?}", s.iter().next(), s.iter().next_back());
    let xs = [1, 2, 3]; println!("{}", xs.iter().rev().zip("abc".chars()).fold(String::new(), |mut a, (n, c)| { a.push(c); a.push_str(&n.to_string()); a }));
    let v: Vec<Option<Result<i32, String>>> = vec![Some(Ok(1)), None, Some(Err("e".into()))]; println!("{:?}", v);
    let w: Vec<Result<i32, String>> = vec![Ok(3), Err("x".into())]; println!("{:?}", w);
    let mut p = Pr { n: 0 }; for _ in 0..2 { match p.parse() { Ok(k) => println!("ok {}", k), Err(m) => println!("err {}", m) } }
}
