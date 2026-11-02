#[derive(Debug)]
enum E { A(Vec<u8>), B, C }
#[derive(Debug)]
struct P { q: i32 }
#[derive(Debug)]
struct M(Option<E>);
#[derive(Debug)]
struct N { o: Option<E>, p: Option<P>, s: Option<String>, n: Option<i32> }
#[derive(Debug)]
struct R { r: Result<P, String> }
fn main() {
    println!("{:?} {:?}", M(Some(E::C)), M(None));
    println!("{:?}", N { o: Some(E::B), p: Some(P { q: 1 }), s: Some(String::from("s")), n: None });
    println!("{:?}", R { r: Ok(P { q: 2 }) });
    println!("{:?}", R { r: Err(String::from("bad")) });
}
