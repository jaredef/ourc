fn rle(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let mut count = 1;
        while i + count < bytes.len() && bytes[i + count] == c {
            count += 1;
        }
        out.push((c as char));
        out.push_str(&count.to_string());
        i += count;
    }
    out
}
fn main() {
    println!("{}", rle("aaabbbbc"));
}
