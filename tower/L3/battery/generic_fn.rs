fn pick<T>(c: bool, a: T, b: T) -> T { if c { a } else { b } }
fn main() { std::process::exit(pick(true, 10, 20) + pick(false, 3, 4)); }   // 10 + 4 = 14
