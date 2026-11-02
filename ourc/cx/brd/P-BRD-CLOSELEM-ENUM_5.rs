#[derive(Clone, Copy)] enum Op { Add = 1, Sub, Mul = 10 }
fn main() { let ops: Vec<Op> = vec![Op::Add, Op::Sub, Op::Mul]; let ds: Vec<u8> = ops.iter().map(|o| *o as u8).collect(); println!("{:?}", ds); }
