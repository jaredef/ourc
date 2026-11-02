// cx10 std fact layer: the std surface rustc's own code leans on -- sort_by/sort_unstable_by_key,
// binary_search, dedup_by_key, retain, BTreeMap/BTreeSet ordered iteration, format width/precision/align,
// char/str methods (split, trim, find, rsplit_once, char_indices), iter windows/chunks/zip/rev/fold/scan.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
fn main() {
    let mut v = vec![("b", 3), ("a", 9), ("c", 1), ("a", 2)]; v.sort_by(|x, y| x.0.cmp(y.0).then(y.1.cmp(&x.1))); println!("{:?}", v);
    let mut w: Vec<i32> = vec![5, 3, 8, 3, 1, 8, 8]; w.sort_unstable(); w.dedup(); println!("{:?} {:?} {:?}", w, w.binary_search(&5), w.binary_search(&4));
    w.retain(|&x| x != 3); println!("{:?}", w);
    let mut m: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for (i, word) in "the quick brown fox jumps over the lazy dog the end".split(' ').enumerate() { m.entry(word.to_string()).or_default().push(i as u32); }
    for (k, v) in m.range("l".to_string()..).take(3) { println!("{:>6}|{:<8}|{:?}", k, k.len(), v); }
    let set: BTreeSet<char> = "mississippi".chars().collect(); println!("{:?} first={:?}", set, set.iter().next());
    let mut out = String::new(); write!(out, "[{:05.2}] [{:+}] [{:^7}] [{:#x}] [{:#b}] [{:e}]", 3.14159, 42, "mid", 255, 5, 1234.5).unwrap(); println!("{}", out);
    let s = "  key = value ; other=thing  "; let parts: Vec<(&str, &str)> = s.trim().split(';').filter_map(|p| p.split_once('=')).map(|(a, b)| (a.trim(), b.trim())).collect(); println!("{:?}", parts);
    println!("{:?} {:?} {:?}", "a/b/c.rs".rsplit_once('/'), "héllo".find('l'), "héllo".char_indices().map(|(i, _)| i).collect::<Vec<_>>());
    let xs = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", xs.windows(3).map(|w| w.iter().sum::<i32>()).collect::<Vec<_>>());
    println!("{:?}", xs.chunks(3).map(|c| c.to_vec()).collect::<Vec<_>>());
    println!("{:?}", xs.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect::<Vec<_>>());
    println!("{}", xs.iter().rev().zip("abcdefg".chars()).fold(String::new(), |mut a, (n, c)| { a.push(c); a.push_str(&n.to_string()); a }));
    println!("{:?} {:?}", xs.iter().position(|&x| x > 4), xs.iter().max_by_key(|&&x| (x % 4, x)));
    let nested: Vec<Option<Result<i32, String>>> = vec![Some(Ok(1)), None, Some(Err("e".into()))]; println!("{:?}", nested);
    println!("{}", ["x", "y", "z"].join("::")); println!("{:?}", "AbC".to_lowercase().repeat(2)); println!("{}", u64::MAX.wrapping_add(2)); println!("{:?}", "10,x,30".split(',').map(|t| t.parse::<i32>()).collect::<Vec<_>>());
}
