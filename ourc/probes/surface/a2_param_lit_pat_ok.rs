fn f(a: i32, b: i32, _: u8) -> i32 { a + b }
fn main() {
    let g = |x: i32, y: i32| x * y;
    println!("{} {}", f(1, 2, 0), g(4, 5));
}
