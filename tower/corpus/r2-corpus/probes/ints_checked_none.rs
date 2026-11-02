fn main() {
    let a: u8 = 200;
    match a.checked_add(100) {
        Some(v) => println!("{}", v),
        None => println!("overflow"),
    }
}
