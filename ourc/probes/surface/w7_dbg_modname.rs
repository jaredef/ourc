mod m1 {
    pub mod m2 {
        #[derive(Debug)]
        pub struct A;
        pub fn mk() -> A { A }
        #[derive(Debug)]
        pub struct P { pub x: i32 }
    }
    #[derive(Debug)]
    pub struct T(pub i32);
    #[derive(Debug)]
    pub struct W<V> { pub v: V }
}
#[derive(Debug)]
struct Top { y: i32 }
fn main() {
    println!("{:?} {:?}", m1::m2::P { x: 1 }, m1::T(2));
    println!("{:?}", Top { y: 3 });
}
