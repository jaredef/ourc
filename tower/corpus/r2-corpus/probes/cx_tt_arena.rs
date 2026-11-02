// P-CX-SLICE-OF-VEC / EXTEND-PEEL / EXTEND-IF / SLICE-SPLIT / REFCELL-METH / RAWPTR-AGG witness (ourc construct-
// generation, 2026-10-25): `args[i]` on `&[Vec<T>]` is a Vec<T>; extend(x.iter().cloned()) and extend(if ..);
// `<vec>.split(|t| ..).map(..).collect()`; `c.borrow_mut().push(x)` / RefCell<HashMap> via borrow(); a raw `*const Adt`
// round trip `&*p` and a generic arena alloc returning through a raw pointer.
use std::cell::RefCell; use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)] enum TT { Tok(String), N(i64) }
fn subst(body: &[TT], args: &[Vec<TT>]) -> Vec<TT> { let mut out = Vec::new(); for t in body { match t { TT::N(i) => out.extend(args[*i as usize].iter().cloned()), _ => out.push(t.clone()) } } out }
fn pick(v: Vec<TT>, deep: bool) -> Vec<TT> { let mut out = Vec::new(); out.extend(if deep { subst(&v, &[vec![TT::Tok("z".into())]]) } else { v }); out }
struct Arena<T> { chunks: RefCell<Vec<Box<T>>> }
impl<T> Arena<T> { fn alloc(&self, t: T) -> &T { let b = Box::new(t); let p: *const T = &*b; self.chunks.borrow_mut().push(b); unsafe { &*p } } fn len(&self) -> usize { self.chunks.borrow().len() } }
fn main() {
    let args = vec![vec![TT::Tok("x".into()), TT::Tok("y".into())], vec![TT::N(7)]];
    println!("{:?}", subst(&[TT::Tok("(".into()), TT::N(0), TT::N(1), TT::Tok(")".into())], &args));
    println!("{:?} {:?}", pick(vec![TT::N(0)], true), pick(vec![TT::N(3)], false));
    let toks = vec![TT::Tok("a".into()), TT::Tok(",".into()), TT::Tok("b".into()), TT::Tok("c".into())];
    let groups: Vec<Vec<TT>> = toks.split(|t| *t == TT::Tok(",".to_string())).map(|g| g.to_vec()).collect(); println!("{} {:?}", groups.len(), groups[1]);
    let c: RefCell<Vec<i32>> = RefCell::new(Vec::new()); c.borrow_mut().push(3); c.borrow_mut().push(4); println!("{} {:?}", c.borrow().len(), c.borrow());
    let s: RefCell<HashMap<TT, u32>> = RefCell::new(HashMap::new()); s.borrow_mut().insert(TT::N(1), 4); println!("{} {:?}", s.borrow().len(), s.borrow().get(&TT::N(1)).copied());
    let k = TT::Tok("raw".into()); let p: *const TT = &k; let r = unsafe { &*p }; println!("{:?}", r);
    let ar = Arena { chunks: RefCell::new(Vec::new()) }; let a1 = ar.alloc(TT::N(5)); let n = match a1 { TT::N(v) => *v, _ => 0 }; println!("{} {}", n, ar.len());
}
