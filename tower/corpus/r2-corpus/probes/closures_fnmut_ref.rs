fn add_all(v: &mut i64, f: impl Fn(i64) -> i64) { *v = f(*v); }
fn main() {
    let mut x = 10;
    add_all(&mut x, |n| n + 3);
    println!("{}", x);
}
