fn main() {
    let a = 'n'; let s = ' '; let d = '5'; let u = '_';
    println!("{}", a.is_ascii_alphabetic());
    println!("{}", a.is_ascii_alphanumeric());
    println!("{}", d.is_ascii_digit());
    println!("{}", s.is_ascii_alphanumeric());
    println!("{}", u.is_ascii_alphanumeric());
    println!("{}", a.is_ascii());
    println!("{}", s.is_whitespace());
}
