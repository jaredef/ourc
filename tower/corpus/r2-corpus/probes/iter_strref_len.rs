fn main() {
    let v = vec!["foo", "bar", "bazz"];
    let mut n = 0;
    for s in &v { n += s.len(); }
    println!("{}", n);
}
