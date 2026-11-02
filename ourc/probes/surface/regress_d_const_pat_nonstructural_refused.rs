#[derive(Debug)]
struct ND(u32);
impl PartialEq for ND { fn eq(&self, _: &Self) -> bool { false } }
const C: ND = ND(1);
fn main() { let x = ND(1); match x { C => println!("c"), _ => println!("o") } }
