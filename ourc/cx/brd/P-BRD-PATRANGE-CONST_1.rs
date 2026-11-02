fn a(n: i32) -> i32 { match n { i32::MIN..=-1 => 1, _ => 0 } }
fn b(n: i32) -> i32 { match n { -100..=-1 => 1, _ => 0 } }
fn c(x: char) -> i32 { match x { '\u{80}'..=char::MAX => 1, _ => 0 } }
fn d(x: u8) -> i32 { match x { 0xf0.. => 4, _ => 0 } }
fn main() { println!("{} {} {} {} {} {}", a(-5), a(3), b(-5), c('é'), c('a'), d(0xf5)); }
