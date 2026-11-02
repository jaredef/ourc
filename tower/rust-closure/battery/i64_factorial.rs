fn fact(n: i64) -> i64 { if n <= 1 { 1 } else { n * fact(n - 1) } }
fn main() { let mut i: i64 = 1; while i <= 20 { println!("{}! = {}", i, fact(i)); i = i + 1; } }
