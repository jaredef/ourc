struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
fn main() { let _a = D("main-end"); println!("x"); }
