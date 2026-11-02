use std::collections::{BTreeSet, HashSet};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)] struct P { x: i32, y: i32 }
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] enum Tok { Num(i64), Id(String), Eof }
fn main() { let bset: BTreeSet<P> = vec![P { x: 2, y: 0 }, P { x: 1, y: 5 }, P { x: 1, y: 5 }].into_iter().collect(); println!("{:?}", bset); }
