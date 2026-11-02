fn sw<T: Clone>(p: (T, T)) -> (T, T) { (p.1.clone(), p.0.clone()) }
fn main() { println!("{}", sw(("x", "y")).0); }
