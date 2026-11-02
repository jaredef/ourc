fn main() {
    let mut v = Vec::new();
    v.push(3);
    v.push(4);
    println!("{} {}", v.len(), v[1]);
    println!("{} {}", i32::MAX, i32::MIN);
    println!("{} {}", u8::MAX, i8::MIN);
    let s = String::new();
    println!("[{}]", s);
}
