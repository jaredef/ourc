fn main() { let base = 100; let f = |x: i64| { let g = |y: i64| y + base; g(x) * 2 }; println!("{}", f(5)); }
