fn tn<T>(_x: T) -> &'static str { std::any::type_name::<T>() }
fn main() { println!("{} {} {}", tn(5i32), tn(String::new()), tn(1u8)); }
