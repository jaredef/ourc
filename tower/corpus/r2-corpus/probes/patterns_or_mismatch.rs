enum E { A(i32), B(i32) }
fn main() {
    let v = E::A(1);
    let r = match v { E::A(n) | E::B(_) => 0 };
    println!("{}", r);
}
