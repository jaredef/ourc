#[derive(Clone, Copy)] enum Op { Add = 1, Sub, Mul = 10, Div }
enum Neg { A = -5, B, C = 100 }
fn main() { println!("{} {} {} {}", Op::Sub as u8, Op::Div as u8, Neg::B as i32, Neg::A as i32); let o = Op::Mul; let r = &o; println!("{}", *r as u8); }
