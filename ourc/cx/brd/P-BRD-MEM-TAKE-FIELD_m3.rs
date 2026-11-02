use std::mem;
fn main() { #[derive(Debug, Default)] struct St { items: Vec<u32>, name: String } let mut st = St { items: vec![4, 5], name: "s".into() }; let t = mem::take(&mut st.items); println!("{:?} {:?}", t, st); }
