#[derive(Debug, Clone, Copy, PartialEq)] struct HirId { owner: u32, local: u32 }
fn main() { let v = vec![(HirId { owner: 1, local: 2 }, 5)]; println!("{:?} {:?}", v[0].0, v); }
