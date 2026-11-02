enum Op { Add(i32, i32), Neg(i32), Zero }
fn eval(o: Op) -> i32 { match o { Op::Add(a, b) => a + b, Op::Neg(x) => -x, Op::Zero => 0 } }
fn main() { std::process::exit(eval(Op::Add(30, 12)) + eval(Op::Neg(5)) + eval(Op::Zero)); }   // 42 - 5 + 0 = 37
