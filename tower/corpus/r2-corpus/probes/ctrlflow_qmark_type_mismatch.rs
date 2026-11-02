fn f(o: Option<i32>) -> Result<i32, String> {
    let x = o?;
    Ok(x)
}
fn main() { println!("{}", f(Some(1)).unwrap()); }
