fn f(x: i64) -> i64 { let y = if x > 3 { return 7 } else { x * 2 }; y + 1 }
fn g() -> i64 { if (return 5) { 1 } else { 2 } }
fn main(){ println!("{} {} {}", f(1), f(9), g()); }
