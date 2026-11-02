const P: *const i32 = 30 as *const i32;
fn main() { let q = 30 as *const i32; let k = match q { P => 1, _ => 0 }; println!("{}", k); }
