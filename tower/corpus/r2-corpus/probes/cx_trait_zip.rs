// P-CX-ZIP-AGG / OPTADT-BINDER / REC-FUSED witness (ourc construct-generation, 2026-10-25): zip(..).all/any over enums
// (had no lowering), `match (pat, ty) { (T::Named(a, xs), ..) }` String binders, `match &s[i] { Some(t) => .. }` over
// Vec<Option<Adt>>, a vec filled through a `&mut Vec<String>` param, a recursive fused filter(..).all(..) predicate
// runs only on kept elements.
#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn k(pat: &T, ty: &T) -> String { match (pat, ty) { (T::Param(i), t) => format!("param{} {:?}", i, t), (T::Named(a, xs), T::Named(b, ys)) => format!("named {} {} {} {}", a, b, xs.len(), ys.len()), _ => "other".into() } }
fn z(xs: &Vec<T>, ys: &Vec<T>) -> bool { xs.iter().zip(ys).all(|(x, y)| x == y) }
fn fill(t: &mut Vec<String>, n: u32) { t.push(format!("x{}", n)); }
fn h(e: &Vec<(u32, u32)>, node: u32, d: u32) -> bool { if d > 3 { return false; } println!("h{} {}", d, node); e.iter().filter(|(p, _)| *p == node).all(|(_, c)| h(e, *c, d + 1)) }
fn main() {
    let i = T::Named("i32".into(), vec![]); println!("{} | {}", k(&T::Param(0), &i), k(&T::Named("Vec".into(), vec![T::Param(0)]), &T::Named("Vec".into(), vec![i.clone()])));
    println!("{} {} {}", z(&vec![i.clone()], &vec![i.clone()]), z(&vec![i.clone()], &vec![T::Param(2)]), vec![1, 2].iter().zip(vec![1, 3].iter()).any(|(a, b)| a != b));
    let s: Vec<Option<T>> = vec![None, Some(T::Param(4))]; match &s[1] { Some(t) => println!("{:?} {}", t, *t == T::Param(4)), None => println!("none") }
    let mut tr = vec![]; fill(&mut tr, 1); fill(&mut tr, 2); println!("{:?}", tr);
    println!("{}", h(&vec![(1u32, 2u32), (2, 3)], 1, 0));
}
