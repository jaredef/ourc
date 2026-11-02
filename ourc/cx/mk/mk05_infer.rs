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
    println!("@0");
    let mut cx = Infcx { parent: vec![], val: vec![], int_kind: vec![] };
    println!("@1");
    // let v = Vec::new(); v.push(1); let f = |x| x; let y = f(v.get(0)); -- the element/closure keystone shape
    println!("@2");
    let elem = cx.new_var(false); let vec_t = Ty::Adt("Vec".into(), vec![elem.clone()]);
    println!("@3");
    let lit = cx.new_var(true); println!("push: {:?}", cx.unify(&elem, &lit));
    println!("@4");
    let a = cx.new_var(false); let r = cx.new_var(false); let clos = Ty::Fn(vec![a.clone()], Box::new(r.clone()));
    println!("@5");
    println!("id-closure: {:?}", cx.unify(&clos, &Ty::Fn(vec![a.clone()], Box::new(a.clone()))));
    println!("@6");
    let opt = Ty::Adt("Option".into(), vec![Ty::Ref(Box::new(elem.clone()))]);
    println!("@7");
    println!("call: {:?}", cx.unify(&a, &opt));
    println!("@8");
    for t in [&vec_t, &clos, &r, &lit] { println!("  {}", show(&cx.resolve(t))); }
    println!("@9");
    let q = cx.new_var(false); println!("occurs: {:?}", cx.unify(&q, &Ty::Adt("Vec".into(), vec![q.clone()])));
    println!("@10");
    let i = cx.new_var(true); println!("int-vs-bool: {:?}", cx.unify(&i, &Ty::Bool));
    println!("@11");
    let (p1, p2) = (cx.new_var(false), cx.new_var(true)); cx.unify(&p1, &p2).unwrap(); println!("var~intvar default: {}", show(&cx.resolve(&p1)));
    println!("@12");
    println!("arity: {:?}", cx.unify(&Ty::Fn(vec![Ty::Int], Box::new(Ty::Int)), &Ty::Fn(vec![], Box::new(Ty::Int))));
    println!("@13");
    println!("vars={} roots={}", cx.parent.len(), (0..cx.parent.len() as u32).filter(|v| cx.find(*v) == *v).count());
}
