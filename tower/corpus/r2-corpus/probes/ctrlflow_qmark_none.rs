fn first_plus(v: &[i32]) -> Option<i32> {
    let x = v.first()?;
    Some(x + 10)
}
fn main() {
    let e: [i32;0] = [];
    println!("{}", first_plus(&e).unwrap_or(-9));
}
