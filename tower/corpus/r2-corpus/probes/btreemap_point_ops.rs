// LAYER2 P6 (slice 1/2): BTreeMap/BTreeSet POINT operations -- new/insert/get/contains_key/len -- reuse the
// HashMap/HashSet storage + dispatch (order-independent, so == rustc without the sort). Ordered iteration
// (`for (k,v) in &m` sorted) is a separate follow-on rung (needs an emitted key-sort at the iteration site).
use std::collections::BTreeMap;
use std::collections::BTreeSet;
fn main() {
    let mut m: BTreeMap<i32, i32> = BTreeMap::new();
    m.insert(3, 30);
    m.insert(1, 10);
    m.insert(2, 20);
    println!("len {}", m.len());
    if let Some(v) = m.get(&2) {
        println!("get2 {}", v);
    }
    println!("has1 {}", m.contains_key(&1));
    println!("has9 {}", m.contains_key(&9));
    let mut s: BTreeSet<i32> = BTreeSet::new();
    s.insert(5);
    s.insert(5);
    s.insert(8);
    println!("set len {}", s.len());
    println!("set has8 {}", s.contains(&8));
}
