fn main() { let w: Vec<((u32, u32), (u32, u32))> = vec![((1, 2), (3, 4))]; let a: Vec<u32> = w.iter().map(|(_, c)| c.0).collect(); let b: Vec<u32> = w.iter().map(|(p, _)| p.0).collect(); println!("{:?} {:?}", a, b); println!("{}", w.iter().all(|(_, c)| c.0 == 3));
  let z: Vec<(u32, u32)> = vec![(1, 2)]; let c: Vec<u32> = z.iter().map(|(_, y)| *y).collect(); println!("{:?}", c); }
