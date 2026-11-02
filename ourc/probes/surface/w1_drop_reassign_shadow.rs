struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } fn main(){ let a = D(1); let b = a; let mut a = D(2); a = D(3); println!("e {}", b.0); }
