// P-BRD-MAP-INSERT-ANDMOD / MEM-TAKE-FIELD / SET-ALGEBRA / SET-INSREM / OPOVL-TY / FROM-OVERLOAD: map insert/remove
// results + entry().and_modify(), mem::take of a struct field, set algebra + predicates + remove, {:?} of an
// overloaded-operator expression, and two From impls on one type chosen by argument type.
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::Add;
#[derive(Debug, Clone, Copy)] struct V(i32, i32);
impl Add for V { type Output = V; fn add(self, o: V) -> V { V(self.0 + o.0, self.1 + o.1) } }
#[derive(Debug)] struct Meters(f64);
impl From<u32> for Meters { fn from(x: u32) -> Self { Meters(x as f64) } }
impl From<&str> for Meters { fn from(s: &str) -> Self { Meters(s.len() as f64) } }
struct St { items: Vec<u32>, n: i32 }
fn main() {
    let mut h: HashMap<String, i32> = HashMap::new();
    h.insert("a".into(), 1);
    let old = h.insert("a".into(), 10);
    h.entry("a".into()).and_modify(|v| *v += 5).or_insert(0);
    println!("{:?} {:?} {:?}", old, h.remove("zz"), h.get("a"));
    let mut st = St { items: vec![4, 5], n: 1 };
    let t = mem::take(&mut st.items);
    println!("{:?} {} {}", t, st.items.len(), st.n);
    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [2, 3, 5].into_iter().collect();
    let i: HashSet<i32> = a.intersection(&b).copied().collect();
    let mut iv: Vec<i32> = i.into_iter().collect(); iv.sort();
    let mut hs: HashSet<&str> = HashSet::new();
    let ins = hs.insert("x"); let ins2 = hs.insert("x"); let rem = hs.remove("x");
    println!("{:?} {} {} {} {} {}", iv, a.is_subset(&b), ins, ins2, rem, hs.len());
    println!("{:?} {:?} {:?}", V(1, 2) + V(3, 4), Meters::from(3u32), Meters::from("abcd"));
}
