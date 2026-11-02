fn main() { for s in ["12", "x", "-4"] { match s.parse::<i64>() { Ok(n) => println!("ok {}", n * 2), Err(_) => println!("bad {}", s) } } }
