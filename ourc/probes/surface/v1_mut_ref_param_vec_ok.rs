fn add(s: &mut String) { s.push_str("bc"); }
fn main() {
    let mut s = String::from("a");
    add(&mut s);
    let mut v = vec![1];
    let r = &mut v;
    r.push(2);
    println!("{} {}", s.len(), v.len());
}
