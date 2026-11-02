fn main() { println!("{:?} {:?} {:?} {:?} {:?}", u32::try_from(-1i64).is_err(), u8::try_from(200i32), i8::try_from(-129i32).is_err(), i64::from_str_radix("-101", 2), u32::from_str_radix("ff", 16)); }
