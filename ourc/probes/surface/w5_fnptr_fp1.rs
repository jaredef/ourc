fn pick(a: &str, b: &str) -> String { if a.len() > b.len() { a.to_string() } else { b.to_string() } } fn main(){ let f: fn(&str, &str) -> String = pick; println!("{}", f("ab", "c")); }
