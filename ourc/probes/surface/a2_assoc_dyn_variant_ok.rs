trait Tr { type Out; fn get(&self) -> i32; }
struct S;
impl Tr for S { type Out = i32; fn get(&self) -> i32 { 4 } }
enum E { A, B }
struct H { e: E }
fn f(x: E) -> i32 { match x { E::A => 1, E::B => 2 } }
fn main() {
    let b: Box<dyn Tr<Out = i32>> = Box::new(S);
    let h = H { e: E::B };
    println!("{} {} {}", b.get(), f(E::A), f(h.e));
}
