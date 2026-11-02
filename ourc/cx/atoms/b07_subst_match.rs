#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn matches(pat: &T, ty: &T, subst: &mut Vec<Option<T>>) -> bool { match (pat, ty) {
  (T::Param(i), t) => match &subst[*i] { Some(prev) => prev == t, None => { subst[*i] = Some(t.clone()); true } },
  (T::Named(a, xs), T::Named(b, ys)) => a == b && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| matches(x, y, subst)), _ => false } }
fn main() { let i32t = T::Named("i32".into(), vec![]); let mut s = vec![None; 1]; let ok = matches(&T::Named("Vec".into(), vec![T::Param(0)]), &T::Named("Vec".into(), vec![i32t.clone()]), &mut s);
  let mut s2 = vec![None; 1]; let ok2 = matches(&T::Named("P".into(), vec![T::Param(0), T::Param(0)]), &T::Named("P".into(), vec![i32t.clone(), T::Named("str".into(), vec![])]), &mut s2); println!("{} {} {}", ok, ok2, s[0] == Some(i32t)); }
