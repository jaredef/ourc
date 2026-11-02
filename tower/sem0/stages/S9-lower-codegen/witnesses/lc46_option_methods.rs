fn main() {
    let a: Option<i32> = Some(7);
    let b: Option<i32> = None;
    let x = if a.is_some() { a.unwrap() } else { 0 };
    let y = if b.is_none() { 100 } else { 0 };
    println!("{}", x + y);
}
