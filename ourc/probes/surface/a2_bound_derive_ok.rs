#[derive(PartialEq)]
struct A(i32);
struct B;
impl PartialEq for B { fn eq(&self, _: &B) -> bool { true } }
fn same<T: PartialEq>(x: T) -> bool { x == x }
fn main() {
    println!("{} {}", same(A(1)), same(B));
}
