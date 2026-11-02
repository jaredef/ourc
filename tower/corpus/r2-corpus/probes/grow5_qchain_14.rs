fn g(x: i32) -> Option<i32> {
    let a = Some(x)?;
    let b = Some(a + 1)?;
    Some(b * 2)
}
fn main() {
    println!("{}", g(5).unwrap_or(-1));
}
