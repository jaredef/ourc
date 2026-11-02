// R2a P6 guard (reject side, rustc E0451): a struct literal naming a PRIVATE field from outside the struct's module.
mod m { pub struct S { pub x: i32, y: i32 } }
fn main() { let s = m::S { x: 1, y: 2 }; println!("{}", s.x); }
