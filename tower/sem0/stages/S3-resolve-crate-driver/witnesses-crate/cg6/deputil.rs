pub struct Sym { pub id: u32 }
impl Sym { pub fn code(&self) -> u32 { self.id * 3 } }
pub fn base(x: u32) -> u32 { x + 1000 }
