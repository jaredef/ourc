fn main() {
    let s = "  the quick  brown fox ";
    let mut n = 0;
    for w in s.split_whitespace() { n += w.len(); }
    println!("{}", n);
}
