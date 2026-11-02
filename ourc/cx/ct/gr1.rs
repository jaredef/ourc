#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { T, L }
fn main() { let edges: Vec<((Q, u32), (Q, u32))> = vec![((Q::L, 1), (Q::T, 1)), ((Q::L, 1), (Q::T, 0))]; let node = (Q::L, 1u32);
  println!("{}", edges.iter().filter(|(p, _)| *p == node).count());
  println!("{}", edges.iter().filter(|(p, _)| *p == node).all(|(_, c)| c.1 < 5));
  let ch: Vec<u32> = vec![2]; println!("{} {}", node.0 == Q::L, ch.contains(&node.1)); }
