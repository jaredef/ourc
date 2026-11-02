fn first_plus(v: &[i32]) -> Option<i32> {
    let x = v.first()?;
    Some(x + 10)
}
fn main() { println!("{}", first_plus(&[5,6]).unwrap()); }
