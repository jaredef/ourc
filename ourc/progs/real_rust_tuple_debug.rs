fn id(v: i32) -> i32 { v }
fn main() {
    println!("{:?} {:?}", (id(1), 2), ());
    println!("{:?}", ("a", 'b', 3, true));
    let s = String::from("hi");
    println!("{:?} {} {}", s, s.len(), s.to_uppercase());
}
