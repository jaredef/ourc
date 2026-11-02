use std::convert::TryFrom;
fn main() { println!("{:?} {:?}", u8::try_from(200i32), u8::try_from(300i32)); println!("{}", f64::from(2.5f32)); println!("{} {}", i64::from(3u8), f64::from(2.5f32) as i64); let small: Result<u8, _> = u8::try_from(300i32); println!("{}", small.is_err()); }
