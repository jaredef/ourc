fn main() {
    let a: Option<i32> = None;
    let b: Option<i32> = Some(5);
    let r = if let Some(x) = a { x } else if let Some(y) = b { y + 100 } else { 0 };
    println!("{}", r);
}
