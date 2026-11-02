use std::mem;
#[derive(Debug, Default)] struct St { items: Vec<u32>, name: String }
fn main() { let mut st = St { items: vec![4, 5], name: "s".into() }; let t = mem::take(&mut st.items); println!("{:?}", t); println!("{:?}", st); }
