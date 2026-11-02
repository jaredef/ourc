fn main() { let v: Vec<f64> = vec![0.1, 1e21, 3.0]; let w: Vec<f64> = vec![1.5, 2.5].iter().map(|x| x * 2.0).collect(); let e: Vec<f64> = Vec::new();
  println!("{:?} {:?} {:?} {:?} {:?}", vec![1.0, 2.5, -0.0], v, w, [0.5, 1.0], e); }
