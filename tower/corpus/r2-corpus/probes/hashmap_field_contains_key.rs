use std::collections::HashMap;
struct I { m: HashMap<String, u32> }
impl I { fn has(&self, s: &str) -> bool { self.m.contains_key(s) } }
fn main() { let mut m: HashMap<String,u32> = HashMap::new(); m.insert(String::from("a"), 5); let it = I { m: m }; println!("{}", it.has("a")); println!("{}", it.has("z")); }
