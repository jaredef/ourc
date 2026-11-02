fn id<T>(x: T) -> T { x } fn main() { let a = id::<i64>(5); let v = Vec::<i64>::new(); println!("{} {}", a, v.len()); }
