fn f(x: u8, y: i64) -> i64 { x as i64 + y }
fn h(z: i32) -> i32 { z * 2 }
fn main() {
    let a: u8 = 3;
    let b: i64 = 4;
    let c: i32 = 5;
    println!("{} {}", f(a, b), h(c));
}
