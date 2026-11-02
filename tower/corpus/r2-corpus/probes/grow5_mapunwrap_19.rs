fn main() {
    let o: Option<i32> = Some(10);
    let n: Option<i32> = None;
    let r = o.map(|x| x * 3).unwrap_or(0) + n.map(|x| x * 3).unwrap_or(7);
    println!("{}", r);
}
