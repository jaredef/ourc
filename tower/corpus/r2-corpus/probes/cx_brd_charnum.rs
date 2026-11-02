// cx_brd_charnum: char ctors + width-aware integer methods + narrow parse/try_from (P-STD-CHAR-NUM, rung363)
fn main() {
    println!("{:?} {:?} {:?} {:?}", char::from_u32(0x41), char::from_u32(0xD800), char::from_digit(11, 16), char::from_digit(10, 10));
    println!("{:?} {:?} {:?} {:?} {:?}", 250u8.checked_add(10), 250u8.checked_add(5), i32::MIN.checked_neg(), 2u32.checked_pow(31), 2u32.checked_pow(32));
    println!("{:?} {:?} {} {} {}", 200u8.overflowing_add(100), 3u8.overflowing_mul(4), 0x1234u16.swap_bytes(), i64::MIN.saturating_abs(), u64::BITS);
    println!("{:?} {} {:?} {:?} {}", "255".parse::<u8>(), "256".parse::<u8>().is_err(), i64::from_str_radix("-101", 2), u8::try_from(200i32), u32::try_from(-1i64).is_err());
}
