fn f(a: (u32, u32)) -> u32 { match a { (0, x) | (x, 0) => x, (x, y) => x * y } }
fn main() { println!("{} {} {}", f((0, 5)), f((6, 0)), f((3, 4))); }
