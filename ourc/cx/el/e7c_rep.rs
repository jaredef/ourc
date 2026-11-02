use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Debug)] struct Sym(u32);
#[derive(Clone, Copy, PartialEq, Debug)] struct P { x: i64, y: i64 }
fn main() { let v = vec![Sym(3)]; let a: Option<&Sym> = v.get(0); println!("{:?}", a);
  let w = vec![P { x: 1, y: 2 }]; let b: Option<&P> = w.get(0); println!("{:?}", b);
  let mut q: HashMap<i64, P> = HashMap::new(); q.insert(1, P { x: 9, y: 8 }); let c: Option<&P> = q.get(&1); println!("{:?}", c); if let Some(p) = q.get(&1) { println!("{} {}", p.x, p.y); }
  let mut r: HashMap<i64, Sym> = HashMap::new(); r.insert(1, Sym(5)); if let Some(s) = r.get(&1) { println!("{}", s.0); } if let Some(&s) = r.get(&1) { println!("{:?}", s); } }
