#[derive(Debug, Clone, Copy, PartialEq)] enum Q { A, B }
fn main() { let v: Vec<(Q, u32)> = vec![(Q::A, 1), (Q::B, 2)]; println!("{} {} {}", v.contains(&(Q::B, 2)), v.contains(&(Q::A, 2)), v.iter().any(|x| *x == (Q::A, 1)));
  let w: Vec<(u32, u32)> = vec![(1, 2)]; println!("{} {}", w.contains(&(1, 2)), w.contains(&(2, 1))); let e = vec![Q::A]; println!("{} {}", e.contains(&Q::A), e.contains(&Q::B)); }
