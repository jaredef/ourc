struct Ctr { v: i32 }
impl Ctr {
    fn cur(&self) -> i32 { self.v }
    fn bump(&self) -> i32 { self.v + 1 }
}
fn main() {
    let c: Ctr = Ctr { v: 40 };
    let x: i32 = c.cur() + c.bump();
    println!("{}", x);
}
