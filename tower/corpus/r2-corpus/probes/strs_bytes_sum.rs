fn main() {
    let mut t = 0u32;
    for b in "abc".bytes() { t += b as u32; }
    println!("{}", t);
}
