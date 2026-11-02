fn half(x: i32) -> Result<i32, String> {
    if x % 2 == 0 { Ok(x/2) } else { Err("odd".into()) }
}
fn main() {
    let r = Ok(16).and_then(half).and_then(half).and_then(half);
    let r2: Result<i32,String> = Ok(12).and_then(half).and_then(half);
    println!("{:?} {:?}", r, r2);
}
