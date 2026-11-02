fn add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}
fn main() {
    println!("{}", add(Some(2), None).unwrap_or(-7));
}
