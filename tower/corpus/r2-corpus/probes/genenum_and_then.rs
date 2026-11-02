fn main() {
    let x: Option<i32> = Some(4);
    let y = x.and_then(|v| if v > 2 { Some(v * 10) } else { None });
    println!("{}", y.unwrap());
}
