struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
fn main() { let _a = D("m1"); for i in 0..2 { let _l = D("loop"); println!("it {}", i); } if true { let _c = D("in-if"); } let _b = D("m2"); }
