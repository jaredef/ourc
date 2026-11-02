struct A { x: u8 }
fn main() { println!("{}", std::mem::size_of::<[A; 2]>()); }
