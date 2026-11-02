// cx05 type inference (crux): rustc_infer-shaped union-find over type variables, structural unify
// with Box/Vec-recursive Ty, occurs check, integer-var defaulting, deep resolve, error reporting.
#[derive(Debug, Clone, PartialEq)]
enum Ty { Int, Bool, Str, IntVar(u32), Var(u32), Fn(Vec<Ty>, Box<Ty>), Adt(String, Vec<Ty>), Ref(Box<Ty>) }
struct Infcx { parent: Vec<u32>, val: Vec<Option<Ty>>, int_kind: Vec<bool> }
impl Infcx {
    fn new_var(&mut self, int: bool) -> Ty { let v = self.parent.len() as u32; self.parent.push(v); self.val.push(None); self.int_kind.push(int); if int { Ty::IntVar(v) } else { Ty::Var(v) } }
    fn find(&mut self, v: u32) -> u32 { let p = self.parent[v as usize]; if p == v { v } else { let r = self.find(p); self.parent[v as usize] = r; r } }
    fn shallow(&mut self, t: &Ty) -> Ty {
        match t { Ty::Var(v) | Ty::IntVar(v) => { let r = self.find(*v); match self.val[r as usize].clone() { Some(x) => self.shallow(&x), None => if self.int_kind[r as usize] { Ty::IntVar(r) } else { Ty::Var(r) } } } _ => t.clone() }
    }
    fn occurs(&mut self, v: u32, t: &Ty) -> bool {
        match self.shallow(t) { Ty::Var(w) | Ty::IntVar(w) => self.find(w) == v, Ty::Fn(a, r) => a.iter().any(|x| self.occurs(v, x)) || self.occurs(v, &r),
            Ty::Adt(_, a) => a.iter().any(|x| self.occurs(v, x)), Ty::Ref(x) => self.occurs(v, &x), _ => false }
    }
    fn unify(&mut self, a: &Ty, b: &Ty) -> Result<(), String> {
        let (a, b) = (self.shallow(a), self.shallow(b));
        match (&a, &b) {
            (Ty::Var(x), Ty::Var(y)) | (Ty::IntVar(x), Ty::IntVar(y)) | (Ty::Var(x), Ty::IntVar(y)) => { let (rx, ry) = (self.find(*x), self.find(*y)); if rx != ry { self.parent[rx as usize] = ry; } Ok(()) }
            (Ty::IntVar(x), Ty::Var(y)) => { let (rx, ry) = (self.find(*x), self.find(*y)); self.parent[ry as usize] = rx; Ok(()) }
            (Ty::Var(x), t) | (t, Ty::Var(x)) => { let r = self.find(*x); if self.occurs(r, t) { return Err(format!("cyclic ?{}", r)); } self.val[r as usize] = Some(t.clone()); Ok(()) }
            (Ty::IntVar(x), t) | (t, Ty::IntVar(x)) => { if *t != Ty::Int { return Err(format!("expected integer, found {:?}", t)); } let r = self.find(*x); self.val[r as usize] = Some(Ty::Int); Ok(()) }
            (Ty::Fn(a1, r1), Ty::Fn(a2, r2)) => { if a1.len() != a2.len() { return Err("arity".into()); } for (x, y) in a1.iter().zip(a2.iter()) { self.unify(x, y)?; } self.unify(r1, r2) }
            (Ty::Adt(n1, a1), Ty::Adt(n2, a2)) if n1 == n2 => { for (x, y) in a1.iter().zip(a2) { self.unify(x, y)?; } Ok(()) }
            (Ty::Ref(x), Ty::Ref(y)) => self.unify(x, y),
            _ if a == b => Ok(()),
            _ => Err(format!("mismatch {:?} vs {:?}", a, b)),
        }
    }
    fn resolve(&mut self, t: &Ty) -> Ty {
        match self.shallow(t) { Ty::IntVar(_) => Ty::Int, Ty::Var(v) => Ty::Var(v), Ty::Fn(a, r) => Ty::Fn(a.iter().map(|x| self.resolve(x)).collect(), Box::new(self.resolve(&r))),
            Ty::Adt(n, a) => Ty::Adt(n, a.iter().map(|x| self.resolve(x)).collect()), Ty::Ref(x) => Ty::Ref(Box::new(self.resolve(&x))), t => t }
    }
}
fn show(t: &Ty) -> String { match t { Ty::Int => "i32".into(), Ty::Bool => "bool".into(), Ty::Str => "&str".into(), Ty::Var(v) => format!("?{}", v), Ty::IntVar(v) => format!("{{int{}}}", v),
    Ty::Fn(a, r) => format!("fn({}) -> {}", a.iter().map(show).collect::<Vec<_>>().join(", "), show(r)), Ty::Adt(n, a) => if a.is_empty() { n.clone() } else { format!("{}<{}>", n, a.iter().map(show).collect::<Vec<_>>().join(", ")) }, Ty::Ref(x) => format!("&{}", show(x)) } }
fn main() {
    let a = Ty::Int; let b = Ty::Adt("Vec".into(), vec![Ty::Int]);
    println!("{}", show(&a)); println!("{}", show(&b));
    for t in [&a, &b] { println!("{}", show(t)); }
    let mut cx = Infcx { parent: vec![], val: vec![], int_kind: vec![] };
    println!("{}", show(&cx.resolve(&b)));
    for t in [&a, &b] { println!("{}", show(&cx.resolve(t))); }
}
