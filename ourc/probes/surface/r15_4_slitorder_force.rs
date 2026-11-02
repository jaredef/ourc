struct S {
    f0: String,
    f1: String,
}
fn tag(n: &mut i32, v: i32) -> i32 {
    *n = *n * 10 + v;
    v
}
struct P {
    a: i32,
    b: i32,
}
fn main() {
    let s = String::from("hi");
    let t = S { f1: s.to_string(), f0: s };
    let mut n = 0;
    let p = P { b: tag(&mut n, 2), a: tag(&mut n, 1) };
    println!("{} {} {} {} {}", t.f0, t.f1, p.a, p.b, n);
}
