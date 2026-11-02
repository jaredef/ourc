fn main() {
    let r: Result<i32, i32> = Err(3);
    let r2 = r.map_err(|e| e + 100);
    println!("{}", r2.unwrap_err());
}
