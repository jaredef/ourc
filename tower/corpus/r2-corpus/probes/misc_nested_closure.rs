fn main() { let add = |a: i64| move |b: i64| a + b; let f = add(10); println!("{}", f(5)); }
