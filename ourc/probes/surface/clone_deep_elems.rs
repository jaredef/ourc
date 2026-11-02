#[derive(Clone, Debug)] struct P { x: i64 } #[derive(Clone, Debug)] struct G { rows: Vec<Vec<i64>> }
fn main() { let a = vec![vec![1], vec![2]]; let mut b = a.clone(); b[0].push(9);
  let c = vec![P { x: 1 }]; let mut d = c.clone(); d[0].x = 5;
  let g = G { rows: vec![vec![1, 2]] }; let mut h = g.clone(); h.rows[0][0] = 7;
  println!("{:?} {:?} {:?} {:?} {:?} {:?}", a, b, c, d, g, h); }
