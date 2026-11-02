struct Seg { name: String }
struct W { segs: Vec<Seg> }
fn go(w: &W) -> String { let mut s = String::new(); for seg in w.segs.iter() { s.push_str(&seg.name); } s }
fn main() { let mut v: Vec<Seg> = Vec::new(); v.push(Seg{name:String::from("a")}); v.push(Seg{name:String::from("b")}); let w = W { segs: v }; println!("{}", go(&w)); }
