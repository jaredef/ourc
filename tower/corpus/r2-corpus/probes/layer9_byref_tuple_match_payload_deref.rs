#[derive(Clone)]
enum Ty { Bool, I32, Infer(u32) }
struct C { table: Vec<Ty> }
fn name(t: &Ty) -> &str { match t { Ty::I32 => "i32", Ty::Bool => "bool", _ => "?" } }
impl C {
    fn resolve(&self, t: &Ty) -> Ty { match t { Ty::Infer(v) => self.table[*v as usize].clone(), _ => t.clone() } }
    fn bind(&mut self, v: u32, t: Ty) { self.table[v as usize] = t; }
}
impl C {
    fn unify(&mut self, a: &Ty, b: &Ty) {
        let ra = self.resolve(a); let rb = self.resolve(b);
        match (&ra, &rb) { (Ty::Infer(v), _) => { self.bind(*v, rb); } _ => {} }
    }
}
fn main() { let mut c = C { table: vec![Ty::Infer(0)] }; c.unify(&Ty::Infer(0), &Ty::I32); println!("{}", name(&c.resolve(&Ty::Infer(0)))); }
