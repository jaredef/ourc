struct P { x: i32, y: i32 }
fn sum(p: &P) -> i32 { p.x + p.y }
fn main() { let a = P { x: 3, y: 4 }; let s1 = sum(&a); let s2 = sum(&a); std::process::exit(s1 + s2); }  // 7+7=14
