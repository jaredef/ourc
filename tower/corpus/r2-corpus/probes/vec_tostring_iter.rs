// vec! of multiple str.to_string()/.to_owned() elements, iterated -- regression
// guard for TI3e: str.to_string()/.to_owned() must dispatch to the identity
// descriptor in dispatch_string, not fall through to the undefined `call
// to_string` (which silently dropped the 2nd+ element).
fn main() {
    let v = vec!["ab".to_string(), "cde".to_string(), "f".to_string()];
    let mut t = 0;
    for s in &v { t = t + s.len(); }
    println!("{}", t);
    println!("{}", v.len());
    let w = vec!["xy".to_owned(), "z".to_owned()];
    let mut joined = String::new();
    for s in &w { joined.push_str(s); }
    println!("{}", joined);
    println!("{}", w.len());
}
