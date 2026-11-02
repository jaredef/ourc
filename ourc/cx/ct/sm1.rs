#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn k(pat: &T, ty: &T) -> String { match (pat, ty) { (T::Param(i), t) => format!("param{} {:?}", i, t), (T::Named(a, xs), T::Named(b, ys)) => format!("named {} {} {} {}", a, b, xs.len(), ys.len()), _ => "other".into() } }
fn z(xs: &Vec<T>, ys: &Vec<T>) -> bool { xs.iter().zip(ys).all(|(x, y)| x == y) }
fn main() { let i = T::Named("i32".into(), vec![]); println!("{}", k(&T::Param(0), &i)); println!("{}", k(&T::Named("Vec".into(), vec![T::Param(0)]), &T::Named("Vec".into(), vec![i.clone()]))); println!("{}", k(&i, &T::Param(1)));
  println!("{} {}", z(&vec![i.clone()], &vec![i.clone()]), z(&vec![i.clone()], &vec![T::Param(2)])); }
