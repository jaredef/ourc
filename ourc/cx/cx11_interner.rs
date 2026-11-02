// cx11 interners/arena: rustc_span Symbol interner (HashMap<String,u32> + Vec<String>), Copy newtypes,
// rustc_middle-shaped Interned<'tcx, T> pointer-equality over a typed arena, CtxtInterners dedup.
use std::collections::HashMap;
use std::cell::RefCell;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)] struct Symbol(u32);
struct Interner { map: HashMap<String, Symbol>, strs: Vec<String> }
impl Interner {
    fn prefill(init: &[&str]) -> Self { let mut i = Interner { map: HashMap::new(), strs: Vec::new() }; for s in init { i.intern(s); } i }
    fn intern(&mut self, s: &str) -> Symbol { if let Some(&sym) = self.map.get(s) { return sym; } let sym = Symbol(self.strs.len() as u32); self.strs.push(s.to_string()); self.map.insert(s.to_string(), sym); sym }
    fn get(&self, s: Symbol) -> &str { &self.strs[s.0 as usize] }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)] enum TyKind { Int, Ref(u32), Adt(Symbol, Vec<u32>), Tuple(Vec<u32>) }
struct Arena<T> { chunks: RefCell<Vec<Box<T>>> }
impl<T> Arena<T> { fn alloc(&self, t: T) -> &T { let b = Box::new(t); let p: *const T = &*b; self.chunks.borrow_mut().push(b); unsafe { &*p } } fn len(&self) -> usize { self.chunks.borrow().len() } }
#[derive(Clone, Copy)] struct Ty<'tcx>(&'tcx TyKind);
impl<'tcx> PartialEq for Ty<'tcx> { fn eq(&self, o: &Self) -> bool { std::ptr::eq(self.0, o.0) } }
struct Ctxt<'tcx> { arena: &'tcx Arena<TyKind>, set: RefCell<HashMap<TyKind, &'tcx TyKind>>, ids: RefCell<Vec<&'tcx TyKind>> }
impl<'tcx> Ctxt<'tcx> {
    fn mk(&self, k: TyKind) -> Ty<'tcx> {
        if let Some(&p) = self.set.borrow().get(&k) { return Ty(p); }
        let p = self.arena.alloc(k.clone()); self.set.borrow_mut().insert(k, p); self.ids.borrow_mut().push(p); Ty(p)
    }
    fn id(&self, t: Ty<'tcx>) -> u32 { self.ids.borrow().iter().position(|p| std::ptr::eq(*p, t.0)).unwrap() as u32 }
}
fn main() {
    let mut i = Interner::prefill(&["", "fn", "let", "self", "Vec"]);
    let syms: Vec<Symbol> = ["foo", "let", "bar", "foo", "Vec", "ünï"].iter().map(|s| i.intern(s)).collect();
    println!("{:?}", syms); println!("{:?} len={}", syms.iter().map(|s| i.get(*s)).collect::<Vec<_>>(), i.strs.len());
    let mut sorted = syms.clone(); sorted.sort(); sorted.dedup(); println!("uniq {:?}", sorted);
    let arena = Arena { chunks: RefCell::new(Vec::new()) };
    let cx = Ctxt { arena: &arena, set: RefCell::new(HashMap::new()), ids: RefCell::new(Vec::new()) };
    let int = cx.mk(TyKind::Int); let int2 = cx.mk(TyKind::Int);
    let r = cx.mk(TyKind::Ref(cx.id(int))); let r2 = cx.mk(TyKind::Ref(cx.id(int2)));
    let vec_sym = i.intern("Vec"); let v = cx.mk(TyKind::Adt(vec_sym, vec![cx.id(r)])); let v2 = cx.mk(TyKind::Adt(vec_sym, vec![cx.id(r2)]));
    let t = cx.mk(TyKind::Tuple(vec![cx.id(v), cx.id(int)])); let t2 = cx.mk(TyKind::Tuple(vec![cx.id(int), cx.id(v)]));
    println!("int==int2 {} r==r2 {} v==v2 {} t==t2 {}", int == int2, r == r2, v == v2, t == t2);
    println!("arena={} ids: {} {} {} {}", arena.len(), cx.id(int), cx.id(r), cx.id(v), cx.id(t2));
    println!("{:?}", t2.0);
}
