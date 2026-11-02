use std::collections::HashMap;
struct Resolver { scopes: Vec<HashMap<i32, u32>>, next: u32 }
impl Resolver {
    fn new() -> Resolver { Resolver { scopes: Vec::new(), next: 0 } }
    fn enter(&mut self) { self.scopes.push(HashMap::new()); }
    fn exit(&mut self) { self.scopes.pop(); }
    fn define(&mut self, name: i32) -> u32 { let id = self.next; self.next = self.next + 1; let t = self.scopes.len() - 1; self.scopes[t].insert(name, id); id }
    fn resolve(&self, name: i32) -> Option<u32> {
        let mut i = self.scopes.len();
        while i > 0 { i = i - 1; if let Some(id) = self.scopes[i].get(&name) { return Some(*id); } }
        None
    }
}
fn main() {
    let mut r = Resolver::new();
    r.enter(); r.define(1); r.define(2);
    r.enter(); r.define(1);
    match r.resolve(1) { Some(d) => println!("{}", d), None => println!("?") }   // 2 (inner)
    match r.resolve(2) { Some(d) => println!("{}", d), None => println!("?") }   // 1
    r.exit();
    match r.resolve(1) { Some(d) => println!("{}", d), None => println!("?") }   // 0 (outer)
}
