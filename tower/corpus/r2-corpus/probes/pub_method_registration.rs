struct Cur { pos: i64 }
impl Cur {
    pub fn new(p: i64) -> Cur { Cur { pos: p } }
    pub(crate) fn at_end(&self) -> bool { self.pos >= 10 }
    fn step(&mut self) -> i64 { self.pos = self.pos + 1; self.pos }
    pub fn label(&self) -> String { String::from("done") }
}
fn main() {
    let mut c = Cur::new(0);
    let mut sum: i64 = 0;
    while !c.at_end() { sum = sum + c.step(); }
    println!("{} {}", sum, c.label());
}
