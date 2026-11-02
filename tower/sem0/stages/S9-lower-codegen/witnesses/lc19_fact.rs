fn fact(n: i32) -> i32 { if n <= 1 { 1 } else { n * fact(n - 1) } }
fn main() {
    let r: i32 = fact(5);
    println!("{}", r);
}
