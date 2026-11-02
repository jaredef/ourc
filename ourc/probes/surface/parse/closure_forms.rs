fn main() { let f = |x: i64| x + 1; let g = |a, b| { a * b }; let h = move || 5; println!("{} {} {}", f(1), g(2, 3), h()); }
