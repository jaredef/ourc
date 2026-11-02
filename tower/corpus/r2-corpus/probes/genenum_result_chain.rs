fn half(x: i32) -> Result<i32, i32> {
    if x % 2 == 0 { Ok(x / 2) } else { Err(x) }
}
fn main() {
    let r = half(8).and_then(half);
    println!("{}", r.unwrap());
}
