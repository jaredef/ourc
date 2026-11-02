pub struct D;
macro_rules! g { (x = $x:expr,) => {{ const A: u32 = 1; const B: u32 = 2; $x }}; }
impl D {
    pub fn m(self) -> u32 { g!(x = 5,) }
}
fn main() {}
