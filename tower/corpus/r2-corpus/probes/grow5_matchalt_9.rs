fn f(n: i32) -> i32 {
    match n {
        1 | 2 | 3 => 100,
        x @ (4 | 5) => x * 10,
        _ => -1,
    }
}
fn main() {
    println!("{}", f(1) + f(5) + f(9));
}
