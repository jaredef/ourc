#[repr(u8)]
enum B { P = 2u8, Q = 9u8 }
enum A { X = 1, Y = 5isize }
fn main() { println!("{} {}", B::Q as u8, A::Y as i32); }
