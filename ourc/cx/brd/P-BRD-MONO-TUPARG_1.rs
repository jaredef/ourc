fn sz<T>(_x: T) -> usize { std::mem::size_of::<T>() }
fn main() { println!("{} {} {} {}", sz(5i32), sz(String::new()), sz(1u8), sz((1u8, true))); }
