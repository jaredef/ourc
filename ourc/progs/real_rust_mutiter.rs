fn main() {
    let mut a = [0; 5];
    a[0] += 2;
    for i in &mut a { *i += 8; }
    let mut s = 0;
    for i in &a { s += *i; }
    println!("{} {}", a.len(), s);
}
