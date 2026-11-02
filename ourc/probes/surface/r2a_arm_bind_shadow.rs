// R2a P4 guard: a match arm's payload binding shadows an outer name of another type (`n: i64` outside, `E::A(n)` a
// String inside). OPT-PAY-ARM typed the arm body WITHOUT the arm's bindings, read `n` as the outer i64, and refused
// the match as "arms have incompatible types" (ourc's own olower::ety_of had this shape).
enum E {
    A(String),
    B,
}
fn f(e: &E, n: i64) -> String {
    let k = n + 1;
    match e {
        E::A(n) => n.clone(),
        E::B => String::from("b"),
    }
}
fn main() {
    println!("{} {}", f(&E::A(String::from("a")), 3), f(&E::B, 4));
}
