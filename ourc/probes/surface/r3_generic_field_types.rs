struct T<A> { x: A, n: i32 }
fn g(t: T<String>) -> usize { t.x.len() + t.n as usize }
fn main() {
    let s = T { x: String::from("abc"), n: 1 };
    let i = T { x: 5, n: 2 };
    let f = T { x: 2.5, n: 3 };
    println!("{} {} {} {}", s.x.len(), i.x + 1, f.x * 2.0, g(T { x: String::from("hey"), n: 2 }));
}
