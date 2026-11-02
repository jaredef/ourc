struct S { x: i32 }
fn main() { let a = S { x: 9 }; let _b = a; let _c = a; std::process::exit(0); }
