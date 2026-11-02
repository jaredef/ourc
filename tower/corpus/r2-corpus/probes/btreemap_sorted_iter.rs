// LAYER2 P6 (slice 2/2): BTreeMap ORDERED iteration -- `for (k,v) in &m` yields entries sorted by key asc.
// btree-ness is marked on the interned Ty (tag 18) at BTreeMap::new, recorded in the local's STY, surfaced at
// the &m read, and read at the for-in site to emit an in-place selection sort (__pair_sort) before iterating.
// HashMap iteration is unaffected. i32 keys (String-key ordering + BTreeSet for-in are later slices).
use std::collections::BTreeMap;
fn main() {
    let mut m: BTreeMap<i32, i32> = BTreeMap::new();
    m.insert(30, 300);
    m.insert(10, 100);
    m.insert(20, 200);
    m.insert(5, 50);
    for (k, v) in &m {
        println!("{} => {}", k, v);
    }
    let mut totals: BTreeMap<i32, i32> = BTreeMap::new();
    let data = [3, 1, 2, 1, 3, 3];
    for &d in data.iter() {
        let e = totals.entry(d).or_insert(0);
        *e += 1;
    }
    for (k, v) in &totals {
        println!("count {} = {}", k, v);
    }
}
