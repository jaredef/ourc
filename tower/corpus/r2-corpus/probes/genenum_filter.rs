fn main() {
    let x: Option<i32> = Some(5);
    let y = x.filter(|&v| v % 2 == 0);
    println!("{}", y.unwrap_or(-1));
}
