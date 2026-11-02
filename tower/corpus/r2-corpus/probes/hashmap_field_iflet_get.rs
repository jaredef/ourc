use std::collections::HashMap;
struct I { m: HashMap<String, u32> }
impl I {
    fn look(&self, s: &str) -> u32 { if let Some(id) = self.m.get(s) { return *id; } 99 }
}
fn main() {
    let mut m: HashMap<String, u32> = HashMap::new(); m.insert(String::from("a"), 5);
    let it = I { m: m };
    println!("{}", it.look("a")); println!("{}", it.look("b"));
}
