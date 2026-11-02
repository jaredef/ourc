use std::mem;
struct St { items: Vec<u32>, n: i32 }
fn main() { let mut st = St { items: vec![4, 5], n: 1 }; let t = mem::take(&mut st.items); println!("{:?} {} {}", t, st.items.len(), st.n); }
