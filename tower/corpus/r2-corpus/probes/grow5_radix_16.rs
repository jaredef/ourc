fn main() {
    let a = i32::from_str_radix("ff", 16).unwrap();
    let b = i32::from_str_radix("1010", 2).unwrap();
    println!("{}", a + b);
}
