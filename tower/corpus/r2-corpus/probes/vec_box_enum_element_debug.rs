#[derive(Debug)]
enum N { Num(i64), Add(Box<N>, Box<N>) }
fn main() {
    let mut v: Vec<Box<N>> = Vec::new();
    v.push(Box::new(N::Add(Box::new(N::Num(4)), Box::new(N::Num(5)))));
    println!("{:?}", v[0]);
}
