fn main() {
    let mut s = 0;
    if (97u8 as char).is_alphabetic() { s += 1; }
    if (53u8 as char).is_numeric() { s += 2; }
    if (32u8 as char).is_whitespace() { s += 4; }
    if (11u8 as char).is_whitespace() { s += 8; }
    if (65u8 as char).is_alphanumeric() { s += 16; }
    if !(46u8 as char).is_alphabetic() { s += 32; }
    println!("{}", s);
}
