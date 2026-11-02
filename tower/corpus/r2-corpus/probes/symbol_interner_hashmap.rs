use std::collections::HashMap;
struct Interner { map: HashMap<String, u32>, names: Vec<String> }
impl Interner {
    fn new() -> Interner { Interner { map: HashMap::new(), names: Vec::new() } }
    fn intern(&mut self, s: &str) -> u32 {
        if let Some(id) = self.map.get(s) { return *id; }
        let id = self.names.len() as u32;
        self.names.push(String::from(s));
        self.map.insert(String::from(s), id);
        id
    }
    fn as_str(&self, id: u32) -> &str { &self.names[id as usize] }
}
fn main() {
    let mut it = Interner::new();
    let a = it.intern("fn");
    let b = it.intern("let");
    let c = it.intern("fn");
    println!("{} {} {}", a, b, c);
    println!("{}", a == c);
    println!("{}", a == b);
    println!("{}", it.as_str(a));
    println!("{}", it.as_str(b));
}
