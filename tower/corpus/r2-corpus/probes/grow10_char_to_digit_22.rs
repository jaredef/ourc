fn main() {
    let s = "12a4";
    let mut sum = 0u32;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) { sum += d; }
    }
    println!("{}", sum);
}
