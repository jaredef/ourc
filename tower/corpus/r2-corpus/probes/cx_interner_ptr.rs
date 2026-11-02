// L1-cx11 witness (ourc construct-generation, 2026-10-25; locales/L1-cx11-interner-arena): RefCell<Vec<&T>> /
// RefCell<HashMap<K,&T>> through borrow_mut (IC-3); std::ptr::eq (IC-6); a generic method reached through a
// `&'tcx Arena<T>` FIELD (IC-7/8) and only from a lifetime-generic method body (IC-9); `==` routed to a USER
// PartialEq (IC-10a); HashMap<K,&T> read-back via `if let Some(&p)` / `.copied()` keeps identity (IC-12).
use std::cell::RefCell; use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Hash, Clone)] enum K { Int, Ref(u32) }
struct Arena<T> { chunks: RefCell<Vec<Box<T>>> }
impl<T> Arena<T> { fn alloc(&self, t: T) -> &T { let b = Box::new(t); let p: *const T = &*b; self.chunks.borrow_mut().push(b); unsafe { &*p } } fn len(&self) -> usize { self.chunks.borrow().len() } }
#[derive(Clone, Copy)] struct Ty<'a>(&'a K);
impl<'a> PartialEq for Ty<'a> { fn eq(&self, o: &Self) -> bool { std::ptr::eq(self.0, o.0) } }
#[derive(Clone, Copy)] struct W(u32);
impl PartialEq for W { fn eq(&self, o: &Self) -> bool { self.0 % 2 == o.0 % 2 } }
struct Cx<'a> { arena: &'a Arena<K>, set: RefCell<HashMap<K, &'a K>>, ids: RefCell<Vec<&'a K>> }
impl<'a> Cx<'a> {
    fn mk(&self, k: K) -> Ty<'a> { if let Some(&p) = self.set.borrow().get(&k) { return Ty(p); } let p = self.arena.alloc(k.clone()); self.set.borrow_mut().insert(k, p); self.ids.borrow_mut().push(p); Ty(p) }
    fn id(&self, t: Ty<'a>) -> usize { self.ids.borrow().iter().position(|p| std::ptr::eq(*p, t.0)).unwrap() }
}
fn main() {
    let arena = Arena { chunks: RefCell::new(Vec::new()) };
    let cx = Cx { arena: &arena, set: RefCell::new(HashMap::new()), ids: RefCell::new(Vec::new()) };
    let a = cx.mk(K::Int); let b = cx.mk(K::Ref(3)); let a2 = cx.mk(K::Int);
    println!("{} {} {} {} {}", a == a2, a == b, arena.len(), cx.arena.len(), cx.id(b));
    let g = cx.set.borrow().get(&K::Int).copied(); println!("{}", std::ptr::eq(g.unwrap(), a.0));
    let (w1, w3, w2) = (W(1), W(3), W(2)); println!("{} {}", w1 == w3, w1 != w2);
}
