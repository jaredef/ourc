// P-CX-REVSTEP / MAPDBG / MAPRANGE / SETDBG witness (ourc construct-generation, 2026-10-25): `.rev().step_by(n)` and
// `.rev().skip(k).step_by(n)` collect; `{:?}` of HashMap/BTreeMap (keys ptr load8; Vec values via the field printer);
// `for (k,v) in m.range(lo..)[.take(n)]` over a BTreeMap; `{:?}` of BTreeSet; set collect sorts + dedups; chars -> set.
use std::collections::{BTreeMap, BTreeSet};
fn main() {
    let arr = [5, 1, 4, 2, 3];
    println!("{:?} {:?}", arr.iter().rev().step_by(2).collect::<Vec<_>>(), arr.iter().rev().skip(1).step_by(2).collect::<Vec<_>>());
    let mut m: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for (i, w) in "the quick lazy over the dog".split(' ').enumerate() { m.entry(w.to_string()).or_default().push(i as u32); }
    println!("{:?}", m);
    for (k, v) in m.range("l".to_string()..).take(2) { println!("{} {:?}", k, v); }
    let mut n: BTreeMap<u32, u32> = BTreeMap::new(); n.insert(3, 30); n.insert(1, 10); n.insert(2, 20);
    for (k, v) in n.range(2..) { println!("{}={}", k, v); }
    println!("{:?}", n);
    let set: BTreeSet<char> = "mississippi".chars().collect(); println!("{:?}", set);
    let s2: BTreeSet<u32> = vec![3, 1, 3, 2, 1].into_iter().collect(); println!("{:?} {}", s2, s2.len());
}
