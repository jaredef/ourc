struct T<A> { x: A, n: i32 }
fn g(t: T<String>) -> i32 { t.n + 7 }
fn main() { println!("{}", g(T { x: String::from("abc"), n: 2 })); }
