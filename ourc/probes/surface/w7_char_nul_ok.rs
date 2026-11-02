fn main() {
    let c = '\0';
    let d = '\x00';
    let s = "a\tb";
    println!("{} {} {}", c == d, c as u32, s.len());
}
