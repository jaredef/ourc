#[derive(Clone, Debug)] struct Inst { def: String }
fn main() { let a = vec![Inst { def: "x".into() }]; println!("{:?}", a); let k = 1; let v = match k { 1 => vec![Inst { def: "y".into() }], _ => vec![] }; println!("{:?}", v); let w = if k == 1 { vec![Inst { def: "z".into() }] } else { Vec::new() }; println!("{:?}", w); }
