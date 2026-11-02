#[derive(Clone)]
enum Ty { Bool, I32, Str, Ref(Box<Ty>), Infer(u32), Error }
struct C { table: Vec<Ty> }
impl C {
    fn fresh(&mut self) -> Ty { let id = self.table.len() as u32; self.table.push(Ty::Infer(id)); Ty::Infer(id) }
    fn bind(&mut self, v: u32, t: Ty) { self.table[v as usize] = t; }
    fn resolve(&self, t: &Ty) -> Ty {
        match t {
            Ty::Infer(v) => { let b = self.table[*v as usize].clone(); match b { Ty::Infer(w) => { if w == *v { Ty::Infer(*v) } else { self.resolve(&Ty::Infer(w)) } } _ => self.resolve(&b) } }
            Ty::Ref(inner) => Ty::Ref(Box::new(self.resolve(inner))),
            _ => t.clone(),
        }
    }
}
fn name(t: &Ty) -> &str { match t { Ty::I32 => "i32", _ => "?" } }
fn main() {
    let mut c = C { table: Vec::new() };
    let v = c.fresh();
    c.bind(0, Ty::I32);
    println!("{}", name(&c.resolve(&v)));
}
