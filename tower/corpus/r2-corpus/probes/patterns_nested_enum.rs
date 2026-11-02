enum E { A(i32), B(i32) }
enum W { X(E), Y }
fn main() {
    let v = W::X(E::B(7));
    let r = match v { W::X(E::B(n)) => n, W::X(E::A(n)) => n+100, W::Y => 0 };
    println!("{}", r);
}
