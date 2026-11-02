struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } fn take(d: D) { println!("took {}", d.0); } fn main() { let a = D(1); take(a); let _b = D(2); println!("end"); }
