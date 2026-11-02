#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn a(s: &Vec<Option<T>>, t: &T) -> bool { match &s[0] { Some(prev) => prev == t, None => false } }
fn b(x: &T, y: &T) -> bool { x == y }
fn c(xs: &Vec<T>, ys: &Vec<T>) -> bool { xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| x == y) }
fn main() { let i = T::Named("i32".into(), vec![]); let s = vec![Some(i.clone())]; println!("{} {} {} {}", a(&s, &i), a(&s, &T::Param(0)), b(&i, &i.clone()), c(&vec![i.clone(), T::Param(1)], &vec![i.clone(), T::Param(1)])); }
