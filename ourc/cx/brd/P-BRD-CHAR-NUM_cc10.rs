fn main() { println!("{:?} {:?} {:?} {:?} {:?}", char::from_u32(0xD800), char::from_digit(11, 16), char::from_digit(10, 10), std::char::from_u32(0x1F600).is_some(), char::from_u32(0x41).unwrap()); }
