// R2a P4 guard: char::from_u32 -- Some for a Unicode scalar value (payload typed char), None for surrogates / > 0x10FFFF;
// a user fn named like the prelude helper does not collide.
fn char_from_u32(_v: u32) -> i32 { 7 }
fn main() {
    for v in [97u32, 0xD800, 0xDFFF, 0xE000, 0x10FFFF, 0x110000, 233, 0x1F600] {
        match char::from_u32(v) { Some(c) => print!("[{}]", c as u32), None => print!("[N]") }
    }
    println!();
    match char::from_u32(65) { Some(c) => println!("{}", c), None => println!("none") }
    let mut s = String::new();
    if let Some(ch) = char::from_u32(0x41 + 1) { s.push(ch); }
    println!("{} {} {}", s, char::from_u32(0x110000).is_none(), char_from_u32(1));
}
