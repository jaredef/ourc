fn pick(c: bool) -> i32 {
    let r: i32;
    if c { r = 1; } else { r = 2; }
    r
}
fn main() {
    let a;
    a = 5;
    let b: i64;
    b = a as i64 * 2;
    println!("{} {} {} {}", a, b, pick(true), pick(false));
}
