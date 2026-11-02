// tractable: TYPE::try_from(v) -> Ok/Err by the target int type's range (u8/i8/u16/i16).
fn main() {
    println!("{}", u8::try_from(300i64).is_err());   // true (>255)
    println!("{}", u8::try_from(200i64).is_err());   // false
    println!("{}", u8::try_from(200i64).unwrap_or(0)); // 200
    println!("{}", i8::try_from(200i64).is_err());   // true (>127)
    println!("{}", i8::try_from(-5i64).is_err());    // false
}
