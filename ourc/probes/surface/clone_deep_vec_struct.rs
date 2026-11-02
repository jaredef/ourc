#[derive(Clone, Debug)] struct In { x: i64 } #[derive(Clone, Debug)] struct Out { i: In, n: String, w: Vec<i64> }
fn main() { let a = Out { i: In { x: 1 }, n: String::from("a"), w: vec![5] }; let mut b = a.clone(); b.i.x = 9; b.n.push('z'); b.w[0] = 7; b.w.push(8); println!("{:?} {:?}", a, b); }
