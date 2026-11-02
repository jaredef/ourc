#[derive(Clone, Copy, PartialEq)]
struct DefId(u32);
struct Rib { bindings: Vec<(i32, DefId)> }   // name (interned as i32) -> DefId
struct Resolver { ribs: Vec<Rib>, next: u32 }
impl Resolver {
    fn new() -> Resolver { Resolver { ribs: Vec::new(), next: 0 } }
    fn push_scope(&mut self) { self.ribs.push(Rib { bindings: Vec::new() }); }
    fn pop_scope(&mut self) { self.ribs.pop(); }
    fn define(&mut self, name: i32) -> DefId {
        let id = DefId(self.next); self.next = self.next + 1;
        let top = self.ribs.len() - 1;
        self.ribs[top].bindings.push((name, id));
        id
    }
    fn resolve(&self, name: i32) -> Option<DefId> {
        let mut ri = self.ribs.len();
        while ri > 0 {
            ri = ri - 1;
            let rib = &self.ribs[ri];
            let mut bi = rib.bindings.len();
            while bi > 0 { bi = bi - 1; let b = rib.bindings[bi]; if b.0 == name { return Some(b.1); } }
        }
        None
    }
}
fn main() {
    let mut r = Resolver::new();
    r.push_scope();
    let x_outer = r.define(100);   // x -> DefId(0)
    r.push_scope();
    let y = r.define(200);          // y -> DefId(1)
    let x_inner = r.define(100);    // x -> DefId(2), shadows outer
    match r.resolve(100) { Some(d) => println!("{}", d.0), None => println!("?") }   // 2 (inner shadow)
    match r.resolve(200) { Some(d) => println!("{}", d.0), None => println!("?") }   // 1
    r.pop_scope();
    match r.resolve(100) { Some(d) => println!("{}", d.0), None => println!("?") }   // 0 (outer, after pop)
    match r.resolve(999) { Some(d) => println!("{}", d.0), None => println!("unresolved") }   // unresolved
}
