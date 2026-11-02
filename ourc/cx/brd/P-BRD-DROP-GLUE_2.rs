struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
fn main() { { let _o: Option<Box<D>> = Some(Box::new(D("ann"))); } { let _p = Some(Box::new(D("inf"))); } { let _q = Some(D("plain")); } }
