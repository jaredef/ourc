fn main() {
    let mut v = Vec::new();
    v.push(3);
    let mut o = None;
    if v.len() > 0 { o = Some(v[0]); }
    println!("{} {}", v.len(), o.unwrap());
}
