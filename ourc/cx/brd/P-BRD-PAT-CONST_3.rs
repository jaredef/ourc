const NAME: &str = "rustc";
fn kw(s: &str) -> u8 { match s { NAME => 3, "" => 0, _ => 9 } }
fn main() { println!("{} {} {}", kw("rustc"), kw(""), kw("x")); }
