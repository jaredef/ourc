fn sw<T: Clone>(p: (T, T)) -> (T, T) { (p.1.clone(), p.0.clone()) }
fn main() { let a = sw((1.5, 2.5)); let b = sw((true, false)); let c = sw(("x", "y")); println!("{} {} {} {} {}", a.0, a.1, b.0, c.0, c.1); }
