fn pick(s: &str, b: bool) -> Option<&str> { if b { Some(s) } else { None } }
fn main() { match pick("hi", true) { Some(x) => println!("{}", x), None => println!("none") } }
