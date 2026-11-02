#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn z(xs: &Vec<T>, ys: &Vec<T>) -> bool { xs.iter().zip(ys).all(|(x, y)| { println!("{:?} {:?} {}", x, y, x == y); x == y }) }
fn main() { let i = T::Param(3); println!("{}", z(&vec![i.clone()], &vec![i.clone()])); }
