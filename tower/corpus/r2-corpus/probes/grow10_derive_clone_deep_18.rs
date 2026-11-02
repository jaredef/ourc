#[derive(Clone)]
struct S { v: Vec<i32>, n: i32 }
fn main() {
    let a = S { v: vec![1,2,3], n: 9 };
    let mut b = a.clone();
    b.v.push(4);
    b.n = 0;
    println!("{} {} {} {}", a.v.len(), a.n, b.v.len(), b.n);
}
