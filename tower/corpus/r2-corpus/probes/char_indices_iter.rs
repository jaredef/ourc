// P-STRING-RESIDUAL: for (i, c) in "lit".char_indices() -> (byte-index, char), ASCII.
fn main() {
    let mut last = 0;
    for (i, _c) in "hello".char_indices() { last = i; }
    println!("{}", last);                        // 4
    let mut s = String::new();
    for (i, c) in "abc".char_indices() { s.push(c); let _ = i; }
    println!("{}", s);                           // abc
}
