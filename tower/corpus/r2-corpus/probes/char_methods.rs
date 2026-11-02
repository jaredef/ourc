// tractable: char methods is_alphabetic / is_alphanumeric / to_ascii_uppercase / to_ascii_lowercase.
fn main() {
    let mut a = 0;
    let mut u = String::new();
    for c in "aB3z!".chars() {
        if c.is_alphabetic() { a += 1; }
        u.push(c.to_ascii_uppercase());
    }
    println!("{} {}", a, u);            // 3 AB3Z!
    let mut n = 0;
    for c in "a1 B!".chars() { if c.is_alphanumeric() { n += 1; } }
    println!("{}", n);                  // 3
    let mut l = String::new();
    for c in "HeLLo".chars() { l.push(c.to_ascii_lowercase()); }
    println!("{}", l);                  // hello
}
