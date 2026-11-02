use std::fmt::Display;
struct W<T: Display + Clone> { v: T, n: i32 }
struct I<T: Iterator<Item = Vec<u8>>> { it: T }
fn main() {
    let w = W { v: String::from("hey"), n: 2 };
    assert_eq!(w.v, "hey");
    let u = W { v: 5, n: 1 };
    println!("{} {} {} {}", w.v, w.v.len(), u.v + 1, w.n + u.n);
    let i = I { it: vec![vec![1u8, 2]].into_iter() };
    let _ = i;
}
