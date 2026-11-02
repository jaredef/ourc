use std::collections::{BTreeSet, HashSet};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)] struct P { x: i32, y: i32 }
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] enum Tok { Num(i64), Id(String), Eof }
fn main() { let toks = vec![Tok::Num(3), Tok::Eof, Tok::Num(3)]; let set: HashSet<Tok> = toks.iter().cloned().collect(); println!("{}", set.len()); }
