fn dup<T: Copy>(x: T) -> (T, T) { (x, x) }
fn main() { println!("{:?} {:?} {:?}", dup(3), dup('z'), dup(true)); }
