#[derive(Debug, Clone, PartialEq)] enum Ty { Int, IntVar(u32), Var(u32) }
struct C { parent: Vec<u32>, val: Vec<Option<Ty>>, ik: Vec<bool> }
impl C { fn find(&mut self, v: u32) -> u32 { let p = self.parent[v as usize]; if p == v { v } else { let r = self.find(p); self.parent[v as usize] = r; r } }
  fn shallow(&mut self, t: &Ty) -> Ty { match t { Ty::Var(v) | Ty::IntVar(v) => { let r = self.find(*v); match self.val[r as usize].clone() { Some(x) => self.shallow(&x), None => if self.ik[r as usize] { Ty::IntVar(r) } else { Ty::Var(r) } } } _ => t.clone() } } }
fn main() { let mut c = C { parent: vec![0, 0, 2], val: vec![Some(Ty::Int), None, None], ik: vec![false, false, true] }; println!("{:?} {:?} {:?}", c.shallow(&Ty::Var(1)), c.shallow(&Ty::IntVar(2)), c.shallow(&Ty::Int)); }
