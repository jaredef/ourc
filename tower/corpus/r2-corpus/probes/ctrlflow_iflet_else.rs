fn main() {
    let o: Option<i32> = Some(7);
    let v = if let Some(x) = o { x * 2 } else { -1 };
    println!("{}", v);
}
