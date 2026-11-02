fn get(n: i64) -> Option<i64> { if n > 0 { Some(n * 2) } else { None } }
fn main() {
    if let Some(v) = get(21) { println!("{}", v); } else { println!("none"); }
    match get(0) { Some(v) => println!("{}", v), None => println!("none") }
}
