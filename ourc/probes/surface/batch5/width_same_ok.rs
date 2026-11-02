fn f(a: u8) -> i32 { let b: u8 = a; let c: i32 = b as i32; c } fn main() { let x: i64 = 5; let y: i64 = x; let z: i32 = x as i32; let w = x; let q: i64 = w; println!("{} {} {} {}", y, z, f(3), q); }
