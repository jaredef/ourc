// R36: `chars().next()` / `chars().nth(k)` now return an Option whose payload is a CHAR
// (Display prints the glyph, and a trailing char method like .to_ascii_uppercase() chains),
// not a raw u8. `.next()` was unimplemented (NOELF); `.nth(k).unwrap()` printed the byte (97).
// bytes().nth() stays a u8. Pervasive: capitalize-first-letter is a canonical string idiom.
fn cap_first(w: &str) -> String {
    let first = w.chars().next().unwrap().to_ascii_uppercase();
    format!("{}{}", first, &w[1..])
}

fn main() {
    let words = vec!["apple", "banana", "cherry"];
    let caps: Vec<String> = words.iter().map(|w| cap_first(w)).collect();
    println!("{:?}", caps);

    let c = "hello".chars().nth(1).unwrap();
    println!("{}", c);

    let b = "hello".bytes().nth(0).unwrap();
    println!("{}", b);
}
