trait Tr { type B; }
struct H<R: Tr> { v: <R as Tr>::B }
struct X; struct Y;
impl Tr for X { type B = u32; }
impl Tr for Y { type B = String; }
fn main() {
    let a: H<X> = H { v: 3 };
    let b: H<Y> = H { v: String::from("s") };
    assert_eq!(a.v, 3);
    println!("{} {}", a.v, b.v);
}
