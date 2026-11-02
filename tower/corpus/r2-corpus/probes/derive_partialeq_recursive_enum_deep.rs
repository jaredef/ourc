#[derive(PartialEq)]
enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn main() {
    let x = E::Add(Box::new(E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(2)))), Box::new(E::Lit(3)));
    let y = E::Add(Box::new(E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(2)))), Box::new(E::Lit(3)));
    let z = E::Add(Box::new(E::Add(Box::new(E::Lit(1)), Box::new(E::Lit(8)))), Box::new(E::Lit(3)));
    println!("{} {} {}", x == y, x == z, x != z);
}
