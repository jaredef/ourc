fn apply_twice<F: FnMut() -> i64>(mut f: F) -> i64 {
    f() + f()
}
fn main() {
    let mut c = 0;
    let r = apply_twice(|| { c += 1; c });
    println!("{}", r);
}
