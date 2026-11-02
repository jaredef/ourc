// rungs 352-357: derived Ord on enums with payloads (cmp + sort), sets of derived-Ord structs/enums, copied()/cloned()
// before chain(once(..)), Debug/clone/== of a generic derive struct, and {:#?} of enum variants.
use std::collections::{BTreeSet, HashSet};
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] enum Tok { Num(i64), Id(String), Op { c: char, prec: u8 }, Eof }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)] struct P { x: i32, y: i32 }
#[derive(Debug, Clone, PartialEq, Default)] struct W<T> { inner: Vec<T>, tag: Option<T> }
fn main() {
    let mut t = vec![Tok::Eof, Tok::Id("b".into()), Tok::Num(3), Tok::Id("a".into()), Tok::Num(-1)];
    t.sort();
    println!("{:?} {:?}", t, Tok::Num(2).cmp(&Tok::Num(5)));
    let bs: BTreeSet<P> = vec![P { x: 2, y: 0 }, P { x: 1, y: 5 }, P { x: 1, y: 5 }].into_iter().collect();
    let hs: HashSet<Tok> = t.iter().cloned().chain(std::iter::once(Tok::Num(3))).collect();
    println!("{:?} {}", bs, hs.len());
    let w: W<u8> = W { inner: vec![1, 2], tag: Some(3) };
    let c = w.clone();
    println!("{:?} {:?} {}", w, W::<u8>::default(), w == c);
    println!("{:#?}", Tok::Op { c: '*', prec: 2 });
}
