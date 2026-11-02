struct T<A> { x: A, n: i32 }
fn g(t: T<i32>) -> i32 { t.x + t.n }
fn main() { println!("{}", g(T { x: 4, n: 3 })); }
