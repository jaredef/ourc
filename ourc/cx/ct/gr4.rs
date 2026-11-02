#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { T, S, L }
struct Tcx { edges: Vec<((Q, u32), (Q, u32))> }
fn green(t: &Tcx, node: (Q, u32), changed: &[u32]) -> bool { println!("g {:?}", node); if node.0 == Q::T && changed.contains(&node.1) { return false; } t.edges.iter().filter(|(p, _)| *p == node).all(|(_, c)| green(t, *c, changed)) }
fn chk(node: (Q, u32), changed: &[u32]) -> (bool, bool, bool) { (node.0 == Q::T, changed.contains(&node.1), node.0 == Q::T && changed.contains(&node.1)) }
fn main() { let t = Tcx { edges: vec![((Q::L, 1), (Q::S, 1)), ((Q::S, 1), (Q::T, 1)), ((Q::L, 3), (Q::T, 2))] };
  println!("{:?} {:?}", chk((Q::T, 2), &[2]), chk((Q::T, 1), &[2]));
  println!("{} {} {}", green(&t, (Q::L, 1), &[2]), 0, 0); }
