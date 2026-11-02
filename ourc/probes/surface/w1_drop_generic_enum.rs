struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } enum Op<T> { No, Yes(T) } fn main(){ { let _k = Op::Yes(D(2)); } println!("e"); }
