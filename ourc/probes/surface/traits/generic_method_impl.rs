struct S<T> { v: T } impl<T: Copy> S<T> { fn get(&self) -> T { self.v } } fn main() { println!("{}", S { v: 42 }.get()); }
