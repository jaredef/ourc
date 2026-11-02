fn f(n:i64)->i64{ match n { 1 => 10, other => other } }
fn main() { println!("{} {}", f(1), f(5)); }
