enum Sign { Pos, Zero, Neg }
fn sign_of(n: i32) -> Sign { if n > 0 { Sign::Pos } else if n == 0 { Sign::Zero } else { Sign::Neg } }
fn main() {
    let s = sign_of(-5);
    let code = match s { Sign::Pos => 1, Sign::Zero => 2, Sign::Neg => 3 };
    std::process::exit(code * 10);   // 30
}
