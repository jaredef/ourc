#[derive(Clone)]
enum Ty { Bool, I32, Infer(u32) }
struct C { table: Vec<Ty> }
fn name(t: &Ty) -> &str { match t { Ty::I32 => "i32", Ty::Bool => "bool", _ => "?" } }
impl C {
    fn fresh(&mut self) -> Ty { let id = self.table.len() as u32; self.table.push(Ty::Infer(id)); Ty::Infer(id) }
    fn bind(&mut self, v: u32, t: Ty) { self.table[v as usize] = t; }
    fn get(&self, i: usize) -> Ty { self.table[i].clone() }
}
fn main() { let mut c = C { table: Vec::new() }; c.fresh(); c.table[0] = Ty::I32; println!("{}", name(&c.get(0))); }
