#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Inst { def: String, args: Vec<String> }
fn main() { let i = Inst { def: "f".into(), args: vec!["T".into()] }; let t = |k: usize| i.args.get(k).cloned().unwrap_or_default(); println!("[{}] [{}]", t(0), t(5));
  let v = match i.def.as_str() { "f" => vec![Inst { def: format!("{}::area", t(0)), args: vec![] }], _ => vec![] }; println!("{:?}", v); }
