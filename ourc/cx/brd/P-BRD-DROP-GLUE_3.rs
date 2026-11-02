struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
fn f() { let _a = D("fn-end"); }
fn main() { f(); }
