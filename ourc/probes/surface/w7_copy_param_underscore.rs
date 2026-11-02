#[derive(Copy, Clone)]
struct Wrapper<T>(T);
#[derive(Copy, Clone)]
struct P { a: i32 }
fn foo(_: Wrapper<i32>, _: P, mut q: P) -> i32 { q.a += 1; q.a }
fn bar(mut x: Wrapper<i32>) -> i32 { x.0 + 1 }
fn main() {
    let w = Wrapper::<i32>(1);
    let p = P { a: 4 };
    println!("{} {} {} {}", foo(w, p, p), p.a, bar(w), w.0);
}
