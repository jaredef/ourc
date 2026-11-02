fn main() { let v: Vec<i64> = Vec::with_capacity(4); let mut w = v; w.extend([1, 2, 3].iter()); w.insert(0, 9); w.retain(|&x| x != 2); println!("{:?} {}", w, w.contains(&3)); }
