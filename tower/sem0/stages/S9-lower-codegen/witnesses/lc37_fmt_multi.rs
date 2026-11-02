fn f(x: i32, y: i32) -> i32 { x + y }
fn main() {
    let a: i32 = 3;
    let b: i32 = 4;
    let c: i32 = 12;
    println!("{} + {} = {}", a, b, c);
    println!("sum={} prod={}", a + b, a * b);
    println!("{} then {}", f(a, b), c);
}
