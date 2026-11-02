// interner idiom: intern(v) -> a u32 id (dedup existing); resolve(id) -> v. rustc_span's Symbol/interner shape.
struct Interner { vals: Vec<i32> }
impl Interner {
    fn new() -> Interner { Interner { vals: Vec::new() } }
    fn intern(&mut self, v: i32) -> u32 {
        let mut i = 0;
        while i < self.vals.len() { if self.vals[i] == v { return i as u32; } i = i + 1; }
        self.vals.push(v);
        (self.vals.len() - 1) as u32
    }
    fn resolve(&self, id: u32) -> i32 { self.vals[id as usize] }
}
fn main() {
    let mut t = Interner::new();
    let a = t.intern(100);
    let b = t.intern(200);
    let c = t.intern(100);  // dedup -> == a
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", t.resolve(b));
}
