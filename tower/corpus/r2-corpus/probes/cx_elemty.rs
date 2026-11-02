// P-CX-ELEMTY witness (ourc construct-generation, 2026-10-24): un-annotated element/payload types of user ADTs.
// Vec::new()+push of an enum, find/first over Vec<Adt> -> Option<&Adt>, HashMap<_, Struct>::get (Some(&cell)),
// *get().unwrap() Copy-out, copied(), mem::replace/take results, clone of an indexed field element.
use std::collections::HashMap;
use std::mem;
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Local(u32), Err }
#[derive(Debug, Clone, Copy, PartialEq)] struct Sym(u32);
#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
struct C { val: Vec<Option<Ty>> }
fn mk(i: u32) -> Res { if i < 2 { Res::Local(i) } else { Res::Err } }
fn main() {
    let mut v = Vec::new(); for i in 0..3 { v.push(mk(i)); } let t = Res::Local(9); v.push(t); println!("{:?}", v);
    println!("{:?} {:?}", v.iter().find(|x| **x == Res::Err), v.first());
    let mut m: HashMap<String, Sym> = HashMap::new(); m.insert("a".to_string(), Sym(3));
    println!("{:?} {:?}", m.get("a"), m.get("a").copied());
    let s = *m.get("a").unwrap(); println!("{:?} {}", s, s.0);
    if let Some(p) = m.get("a") { println!("{}", p.0); }
    let w = vec![Sym(5)]; let z = *w.get(0).unwrap(); println!("{:?}", z);
    let mut a = vec![1, 2]; let old = mem::replace(&mut a, vec![7]); let tk = mem::take(&mut a); println!("{:?} {:?} {:?}", a, old, tk);
    let c = C { val: vec![None, Some(Ty::Var(4))] }; println!("{:?}", c.val[1].clone());
}
