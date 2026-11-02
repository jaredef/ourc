struct D(i64); impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } } struct W<T> { x: T } fn main(){ { let _w = W { x: D(3) }; } println!("e"); }
