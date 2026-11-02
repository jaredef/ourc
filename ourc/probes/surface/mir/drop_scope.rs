struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } fn main() { { let _a = D(1); } println!("mid"); let _b = D(2); }
