macro_rules! h { ($e:expr) => { { let x = 10; $e + x } }; } fn main() { let x = 1; println!("{}", h!(x)); }
