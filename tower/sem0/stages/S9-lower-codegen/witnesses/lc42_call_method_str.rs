struct B { }
impl B { fn tag(&self) -> String { let mut s = String::new(); s.push_str("T"); s } }
fn mk() -> String { let mut s = String::new(); s.push_str("M"); s }
fn main() {
    let d = mk();
    println!("{}", d);
    println!("val {}", mk());
    let b = B { };
    println!("{}", b.tag());
}
