#[derive(Debug, Clone, Copy, PartialEq)] struct HirId { owner: u32, local: u32 }
enum H { Lit(i64), Break }
struct L { nodes: Vec<(HirId, H)>, owner: u32, next: u32 }
impl L { fn alloc(&mut self, h: H) -> usize { let id = HirId { owner: self.owner, local: self.next }; self.next += 1; self.nodes.push((id, h)); self.nodes.len() - 1 } }
fn main() { let mut l = L { nodes: vec![], owner: 3, next: 0 }; l.alloc(H::Lit(1)); let r = l.alloc(H::Break); println!("{} {:?}", l.nodes.len(), l.nodes[r].0); }
