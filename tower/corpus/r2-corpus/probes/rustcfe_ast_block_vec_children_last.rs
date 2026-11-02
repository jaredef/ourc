struct Block { stmts: Vec<i32> }
impl Block {
    fn new() -> Block { Block { stmts: Vec::new() } }
    fn push(&mut self, s: i32) { self.stmts.push(s); }
    fn sum(&self) -> i32 { let mut t = 0; let mut i = 0; while i < self.stmts.len() { t = t + self.stmts[i]; i = i + 1; } t }
    fn last(&self) -> Option<&i32> { self.stmts.last() }
}
fn main() { let mut b = Block::new(); b.push(10); b.push(20); b.push(30);
    println!("{}", b.sum());
    match b.last() { Some(x) => println!("{}", *x), None => println!("none") } }
