// P-CX-OPTVEC witness (ourc construct-generation, 2026-10-25): an unannotated `vec![None; n]` filled through a
// `&mut Vec<Option<Adt>>` param (in an `if !f(&mut s)` condition) takes Vec<Option<Adt>> instead of the Vec<i32>
// default, and a closure capturing it sees the refined type (`match &s[*p] { Some(t) => .. }` inside `.all`).
#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn f(subst: &mut Vec<Option<T>>) -> bool { subst[0] = Some(T::Param(3)); subst[1] = Some(T::Named("i32".into(), vec![])); true }
fn g(t: &T) -> bool { println!("g {:?}", t); matches!(t, T::Param(_)) }
fn main() {
    let mut s = vec![None; 3]; if !f(&mut s) { return; } println!("{:?}", s);
    let w: Vec<(String, usize)> = vec![("a".into(), 0), ("b".into(), 1), ("c".into(), 2)];
    let ok = w.iter().all(|(_, p)| match &s[*p] { Some(t) => g(&t.clone()), None => false });
    let any = w.iter().any(|(_, p)| s[*p].is_none());
    println!("{} {}", ok, any);
}
