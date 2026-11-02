fn f(n: i32) -> i32 {
    for i in 0..n {
        if i * i > 10 { return i; }
    }
    -1
}
fn main() { println!("{}", f(100)); }
