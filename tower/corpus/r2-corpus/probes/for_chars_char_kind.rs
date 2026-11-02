// R22: `for c in s.chars()` must bind the loop var as a char -- Display prints the character (was the int
// code) and char-literal compares work. The loop var was untagged before.
fn main() {
    let s = "hi";
    for c in s.chars() { print!("{}", c); }
    println!();
    let t = "aXbYcZ";
    let mut n = 0;
    for c in t.chars() { if c == 'X' || c == 'Y' || c == 'Z' { n += 1; } }
    let u = String::from("Go!");
    for c in u.chars() { print!("{}", c); }
    println!();
    println!("{}", n);
}
