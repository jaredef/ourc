// P-BRD-CHARS-MAP-COLLECT: fused `s.chars().map(f).collect()` / `s.bytes().map(f).collect()` into a Vec.
fn kind(c: char) -> &'static str { match c { 'a'..='z' => "lower", 'A'..='Z' => "upper", _ => "other" } }
fn main() {
    let s = "aZ9";
    let v: Vec<&str> = s.chars().map(kind).collect();
    println!("{:?}", v);
    println!("{:?}", "hé".chars().map(|c| c as u32).collect::<Vec<_>>());
    let b: Vec<u8> = "ab".bytes().map(|x| x + 1).collect();
    println!("{:?} {}", b, "AZ".chars().map(|c| c.to_ascii_lowercase()).collect::<String>());
}
