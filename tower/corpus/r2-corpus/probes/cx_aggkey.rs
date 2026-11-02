// P-CX-HASHMAP-AGGKEY witness (ourc construct-generation, 2026-10-24): HashMap keyed by tuples of enums/ints/Strings
// (query-cache / interner shape), Vec::contains and == over tuples. Also the 64-bit descriptor-pointer fix: the
// struct/tuple key descriptor's kinds pointer was a 32-bit slot, truncated on the LLVM build's mmap heap.
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { A, B }
#[derive(Debug, Clone, PartialEq, Eq, Hash)] struct K { a: u32, b: u32 }
fn main() {
    let mut m: HashMap<(Q, u32), i64> = HashMap::new(); m.insert((Q::A, 1), 10); m.insert((Q::B, 1), 20); m.insert((Q::A, 1), 30);
    println!("{:?} {:?} {:?} {}", m.get(&(Q::A, 1)), m.get(&(Q::B, 1)), m.get(&(Q::B, 2)), m.len());
    let mut s: HashMap<(String, u32), u32> = HashMap::new(); s.insert(("a".to_string(), 2), 1); s.insert(("a".to_string(), 3), 2); s.insert(("a".to_string(), 2), 5);
    println!("{} {:?}", s.len(), s.get(&("a".to_string(), 2)));
    let mut k: HashMap<K, i64> = HashMap::new(); k.insert(K { a: 1, b: 2 }, 10); k.insert(K { a: 1, b: 3 }, 20); k.insert(K { a: 1, b: 2 }, 30); println!("{} {:?}", k.len(), k.get(&K { a: 1, b: 3 }));
    let v: Vec<(Q, u32)> = vec![(Q::A, 1), (Q::B, 2)]; println!("{} {} {}", v.contains(&(Q::B, 2)), v.contains(&(Q::A, 2)), v.iter().any(|x| *x == (Q::A, 1)));
}
