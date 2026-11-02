fn add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}
fn main() {
    println!("{}", add(Some(2), Some(3)).unwrap());
}
