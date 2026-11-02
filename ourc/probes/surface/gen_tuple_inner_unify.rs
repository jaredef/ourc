fn sw<T: Clone>(p: (T, T)) -> (T, T) { (p.1.clone(), p.0.clone()) }
fn fst<T: Clone>(p: (T, i64)) -> T { p.0.clone() }
fn both<A: Clone, B: Clone>(p: (A, B)) -> B { p.1.clone() }
fn main() { let a = sw((1, 2)); let s = sw((String::from("x"), String::from("y"))); println!("{} {} {} {} {} {}", a.0, a.1, s.0, s.1, fst((7, 9)), both((3, String::from("b")))); }
