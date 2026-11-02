#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32), Adt(String, Vec<Ty>) }
struct C { parent: Vec<u32>, val: Vec<Option<Ty>> }
impl C { fn find(&mut self, v: u32) -> u32 { let p = self.parent[v as usize]; if p == v { v } else { let r = self.find(p); self.parent[v as usize] = r; r } } }
fn main() { let mut c = C { parent: vec![0, 0, 1], val: vec![None, Some(Ty::Int), None] }; let r = c.find(2); c.val[2] = Some(Ty::Adt("V".into(), vec![Ty::Var(r)])); println!("{} {:?} {:?}", r, c.parent, c.val[2].clone()); }
