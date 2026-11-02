struct P { x: i64 } fn f(p: P) -> i64 { p.x } fn main() { let p = P { x: 42 }; let c = true; let r = if c { f(p) } else { 0 }; println!("{}", r); }
