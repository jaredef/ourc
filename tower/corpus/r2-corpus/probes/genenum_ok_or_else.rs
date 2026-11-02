fn main() {
    let x: Option<i32> = None;
    let r: Result<i32, i32> = x.ok_or_else(|| 5);
    println!("{}", r.unwrap_err());
}
