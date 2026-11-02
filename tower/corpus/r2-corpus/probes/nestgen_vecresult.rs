fn main() {
    let v: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(2)];
    let s: i32 = v.iter().filter_map(|r| r.ok()).sum();
    println!("{}", s);
}
