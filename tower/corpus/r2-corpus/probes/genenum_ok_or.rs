fn main() {
    let x: Option<i32> = Some(7);
    let r: Result<i32, i32> = x.ok_or(0);
    println!("{}", r.unwrap());
}
