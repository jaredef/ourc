fn f(n: i32) -> i32 {
    match n {
        0 => return 100,
        _ => {}
    }
    n + 1
}
fn main() { println!("{}", f(0) + f(5)); }
