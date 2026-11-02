#[derive(Debug)]
struct W<V> { v: V }
mod m {
    #[derive(Debug)]
    pub struct G<T> { pub t: T, pub n: i32 }
}
#[derive(Debug)]
struct P<A, B> { a: A, b: B }
fn main() {
    let w = W { v: String::from("s") };
    println!("{:?} {:?}", w, W { v: 5 });
    println!("{:?}", m::G { t: true, n: 2 });
    let p = P { a: 1, b: 'x' };
    println!("{:?}", p);
}
