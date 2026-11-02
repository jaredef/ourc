enum E { A(i32), B }
fn main() {
    let e: E = E::A(7);
    let x: i32 = match e { E::A(v) => v + 1, E::B => 0 };
    println!("{}", x);
}
