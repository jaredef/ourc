struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } fn mk() -> D { let x = D(1); x } fn main(){ let y = mk(); println!("e {}", y.0); }
