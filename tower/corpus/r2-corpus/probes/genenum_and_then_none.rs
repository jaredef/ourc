fn main() {
    let x: Option<i32> = None;
    let y = x.and_then(|v| Some(v + 1));
    println!("{}", y.unwrap_or(99));
}
