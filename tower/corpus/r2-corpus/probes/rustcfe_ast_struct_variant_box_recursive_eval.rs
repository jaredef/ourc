enum E { L(i32), B { lhs: Box<E>, rhs: Box<E> } }
impl E {
    fn eval(&self) -> i32 {
        match self {
            E::L(n) => *n,
            E::B { lhs, rhs } => lhs.eval() + rhs.eval(),
        }
    }
}
fn main() { let e = E::B { lhs: Box::new(E::L(3)), rhs: Box::new(E::L(4)) }; println!("{}", e.eval()); }
