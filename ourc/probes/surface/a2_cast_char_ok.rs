fn main() {
    let a = 65u8 as char;
    let b = 66 as u8 as char;
    let n: u32 = 67;
    let c = char::from_u32(n).unwrap();
    println!("{} {} {}", a, b, c);
}
