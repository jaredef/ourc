enum Op { Push(i64), Add, Mul, Dup }
fn run(prog: &[Op]) -> i64 {
    let mut st: Vec<i64> = Vec::new();
    for op in prog {
        match op {
            Op::Push(n) => st.push(*n),
            Op::Add => { let b = st.pop().unwrap(); let a = st.pop().unwrap(); st.push(a + b); }
            Op::Mul => { let b = st.pop().unwrap(); let a = st.pop().unwrap(); st.push(a * b); }
            Op::Dup => { let a = *st.last().unwrap(); st.push(a); }
        }
    }
    st.pop().unwrap()
}
fn main() {
    let prog = [Op::Push(3), Op::Dup, Op::Mul, Op::Push(7), Op::Add];
    println!("{}", run(&prog));
}
