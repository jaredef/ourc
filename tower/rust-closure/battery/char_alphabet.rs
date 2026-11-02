fn main() {
    let mut i = 0;
    while i < 26 { print!("{}", (65 + i) as u8 as char); i = i + 1; }
    println!("");
    let mut j = 0;
    while j < 26 { print!("{}", (97 + j) as u8 as char); j = j + 1; }
    println!("");
}
