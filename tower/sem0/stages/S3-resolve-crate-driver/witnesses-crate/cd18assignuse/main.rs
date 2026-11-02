pub struct S;
impl S {
    pub fn m(self, x: u32) {
        let mut ctor = 0;
        match x {
            0 => { ctor = match x { 1 => { use a::b; 5 } _ => 6 }; }
            _ => { ctor = match x { 1 => { use c::d; 7 } _ => 8 }; }
        }
    }
}
fn main() {}
