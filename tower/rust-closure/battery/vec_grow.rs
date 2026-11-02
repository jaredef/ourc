fn main() {
    let mut v = Vec::new();
    let mut i = 0;
    while i < 100 { v.push(i * i); i = i + 1; }
    println!("len {} first {} last {}", v.len(), v[0], v[99]);
    let mut j = 0; let mut s = 0;
    while j < v.len() { s = s + v[j]; j = j + 1; }
    println!("sum of squares 0..99 = {}", s);
}
