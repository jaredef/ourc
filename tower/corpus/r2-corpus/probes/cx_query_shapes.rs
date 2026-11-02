// P-CX-QUERY witness (ourc construct-generation, 2026-10-25): query-engine shapes -- filter(..).all/any(..) fusion
// calls the predicate (filtered-out elements are neutral), a tuple sort key (`|(q, k)| (*q as u8, *k)`) compares
// lexicographically, `*q as u8` of an &enum reads the tag, `{:?}` of a &(..), captured tuple equality in a filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] enum Q { T, S, L }
fn main() {
    let v = vec![1, 2, 3]; println!("{} {} {}", v.iter().filter(|x| **x > 5).all(|x| *x > 100), v.iter().filter(|x| **x > 1).all(|x| *x > 1), v.iter().filter(|x| **x > 1).any(|x| *x == 3));
    let x = Q::L; let r = &x; println!("{} {}", x as u8, *r as u8);
    let p: Vec<(Q, u32)> = vec![(Q::S, 2), (Q::T, 9), (Q::S, 1)]; for (q, k) in p.iter() { print!("{}:{} ", *q as u8, k); } println!();
    let mut keys: Vec<&(Q, u32)> = p.iter().collect(); keys.sort_by_key(|(q, k)| (*q as u8, *k)); println!("{:?} {:?}", keys[0], keys[keys.len() - 1]);
    let edges: Vec<((Q, u32), (Q, u32))> = vec![((Q::L, 1), (Q::S, 1)), ((Q::L, 1), (Q::T, 0)), ((Q::S, 1), (Q::T, 1))]; let node = (Q::L, 1u32);
    println!("{} {}", edges.iter().filter(|(p, _)| *p == node).count(), edges.iter().filter(|(p, _)| *p == node).all(|(_, c)| c.1 < 5));
}
