fn main() {
    let mut s = String::from("a");
    let r = &mut s;
    r.push_str("bc");
    println!("{}", s.len());
}
