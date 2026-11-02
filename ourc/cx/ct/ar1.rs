fn main() { let pa = [(2usize, "helper"), (3, "x")]; for (m, p) in pa.iter() { println!("{} {} {}", m, p, p.len()); }
  let pv = vec![(2usize, "helper"), (3, "x")]; for (m, p) in pv.iter() { println!("{} {} {}", m, p, p.len()); } }
