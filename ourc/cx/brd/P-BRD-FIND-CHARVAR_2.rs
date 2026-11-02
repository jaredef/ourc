fn sp(s: &str, c: char) -> (&str, &str) { match s.find(c) { Some(i) => (&s[..i], &s[i + 1..]), None => (s, "") } }
fn main() { let (a, b) = sp("key=val", '='); println!("[{}|{}]", a, b); let (c, d) = sp("none", ':'); println!("[{}|{}]", c, d); }
