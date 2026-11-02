struct W<T> { v: T } fn main() { let a = W { v: 40 }; let b = W { v: 2 }; println!("{}", a.v + b.v); }
