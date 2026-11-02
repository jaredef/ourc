fn f(x: i32) -> i32 {
    let y = x + 1;
    y
}
fn g(x: i32) -> i32 {
    if x > 0 {
        return 1;
    }
    0
}
fn h() -> i32 {
    loop {
        return 5;
    }
}
fn main() {
    println!("{} {} {}", f(1), g(2), h());
}
