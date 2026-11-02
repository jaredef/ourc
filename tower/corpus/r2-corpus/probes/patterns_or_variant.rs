enum E { A(i32), B(i32), C(i32) }
fn main() {
    let v = E::B(9);
    let r = match v { E::A(n) | E::B(n) => n, E::C(n) => n+1 };
    println!("{}", r);
}
