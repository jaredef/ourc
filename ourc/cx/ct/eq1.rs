#[derive(Clone, Debug, PartialEq)] struct Inst { def: String, args: Vec<String> }
fn main() { let a = Inst { def: "a".into(), args: vec![] }; let b = Inst { def: "x".into(), args: vec!["i".into()] }; let c = a.clone(); println!("{} {} {}", a == c, a == b, b == b.clone()); let e: Vec<String> = vec![]; let f: Vec<String> = vec!["i".into()]; println!("{} {}", e == f, e == e.clone()); }
