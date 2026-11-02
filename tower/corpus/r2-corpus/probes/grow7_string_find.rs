fn main() {
    let s = "abcdef";
    match s.find('d') {
        Some(i) => println!("{}", i),
        None => println!("none"),
    }
}
