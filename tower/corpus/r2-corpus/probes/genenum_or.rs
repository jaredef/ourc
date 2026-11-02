fn main() {
    let x: Option<i32> = None;
    let y = x.or(Some(9));
    println!("{}", y.unwrap());
}
