#[derive(Clone, Debug)] struct Inst { def: String, args: Vec<String> }
fn main() { let i = Inst { def: "f".into(), args: vec!["T".into()] }; let x = i.args.get(0).cloned().unwrap_or_default(); println!("[{}]", x); let t = |k: usize| i.args.get(k).cloned().unwrap_or_default(); let y = t(0); println!("[{}] [{}]", y, t(0)); }
