use std::convert::TryFrom;
fn main() {
    let r = u8::try_from(300i64);
    println!("{}", r.is_err());
}
