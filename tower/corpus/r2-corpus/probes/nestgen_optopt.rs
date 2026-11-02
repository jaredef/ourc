fn main() {
    let x: Option<Option<i32>> = Some(Some(42));
    let f = x.flatten();
    println!("{}", f.unwrap());
}
