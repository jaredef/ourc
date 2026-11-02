fn f(s: &str, c: char) -> usize { match s.find(c) { Some(i) => i, None => 99 } }
fn main() { println!("{} {}", f("key=val", '='), f("none", ':')); }
