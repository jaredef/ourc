#[derive(Clone, Debug, PartialEq)] struct Inst { def: String, args: Vec<String> }
fn main() { let e: Vec<String> = vec![]; let f: Vec<String> = vec!["i".into()]; println!("{}", f == e); let a = Inst { def: "x".into(), args: vec!["i".into()] }; let b = Inst { def: "a".into(), args: vec![] }; println!("{}", a == b); }
