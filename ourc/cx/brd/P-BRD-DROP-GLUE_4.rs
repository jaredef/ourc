struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
fn main() { { let _q = Some(D("plain")); } }
