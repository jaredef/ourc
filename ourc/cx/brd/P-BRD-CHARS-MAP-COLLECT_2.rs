fn ch(c: char) -> &'static str { match c { 'a'..='z' => "lower", _ => "other" } }
fn by(b: u8) -> u8 { if b > 127 { 1 } else { 0 } }
fn main() { println!("{:?}", "ae".bytes().map(by).collect::<Vec<_>>()); }
