#[derive(Clone, Copy, PartialEq, Debug)] struct Sym(u32);
fn main() { let v = vec![Sym(3)]; let s: Sym = *v.get(0).unwrap(); println!("{:?} {}", s, s.0); if let Some(&t) = v.get(0) { println!("{:?}", t); } let c = v.get(0).copied(); println!("{:?}", c); let u = v.get(0); println!("{:?}", u); }
