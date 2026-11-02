fn repeat(s: &str, n: i32) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < n { out.push_str(s); i = i + 1; }
    out
}
fn main() {
    let line = repeat("=", 10);
    println!("{}", line);
    println!("width {}", line.len());
    let mut row = String::new();
    let mut k = 0;
    while k < 5 { row.push('*'); row.push(' '); k = k + 1; }
    println!("{}", row);
}
