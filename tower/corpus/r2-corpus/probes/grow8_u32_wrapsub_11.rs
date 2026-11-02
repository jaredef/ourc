fn main() {
    let a: u32 = 3;
    let b: u32 = 10;
    let c = a.wrapping_sub(b);
    println!("{}", c);
}
