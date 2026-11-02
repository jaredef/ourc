use std::fmt::Display;
fn show(iter: &Vec<impl Display>) -> String {
    let mut s = String::new();
    for it in iter {
        s.push_str(&format!("{} ", it));
    }
    s
}
fn first(v: Vec<impl Display>) -> String { format!("{}", v[0]) }
fn main() {
    println!("{}", show(&vec![1i32, 2]));
    println!("{}", show(&vec!["a", "b"]));
    println!("{} {}", first(vec![7u32]), first(vec![String::from("x")]));
}
