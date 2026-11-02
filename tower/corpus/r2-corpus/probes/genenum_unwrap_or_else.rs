fn main() {
    let x: Option<i32> = None;
    let v = x.unwrap_or_else(|| 42);
    println!("{}", v);
}
