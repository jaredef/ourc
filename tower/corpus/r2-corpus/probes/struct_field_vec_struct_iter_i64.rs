struct Seg { n: i64 }
struct W { segs: Vec<Seg> }
fn tot(w: &W) -> i64 { let mut t = 0; for s in w.segs.iter() { t = t + s.n; } t }
fn main() { let mut v: Vec<Seg> = Vec::new(); v.push(Seg{n:3}); v.push(Seg{n:4}); let w = W { segs: v }; println!("{}", tot(&w)); }
