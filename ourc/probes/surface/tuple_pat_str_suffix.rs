fn f(s: &str, n: u8) -> i32 { match (s, n) { ("go", 0u8) => 1, ("go", _) => 2, (_, 7u8) => 3, _ => 4 } }
fn main(){ println!("{} {} {} {}", f("go", 0), f("go", 5), f("no", 7), f("no", 1)); }
