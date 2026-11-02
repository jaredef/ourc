#[derive(PartialEq)]
enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn main() {
    let a = E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(2)));
    let b = E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(9)));
    let c = E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(2)));
    println!("{} {} {}", a == b, a == c, E::Lit(3) == E::Lit(3));
    println!("{}", E::Lit(3) == E::Lit(4));
}
