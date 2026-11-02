#[derive(Debug, Clone, Copy, PartialEq)] struct Id { a: u32 }
enum H { Lit(i64), Path(String) }
fn main() { let nodes: Vec<(Id, H)> = vec![(Id { a: 1 }, H::Lit(5))]; println!("{}", nodes[0].0.a); println!("{}", match &nodes[0].1 { H::Lit(n) => *n, _ => 0 });
  let v: Vec<(u32, u32)> = vec![(7, 8)]; println!("{} {}", v[0].0, v[0].1); }
