macro_rules! op { (plus $a:expr, $b:expr) => { $a + $b }; (minus $a:expr, $b:expr) => { $a - $b }; } fn main() { println!("{} {}", op!(plus 4, 3), op!(minus 4, 3)); }
