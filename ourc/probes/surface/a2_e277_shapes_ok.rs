fn main() {
    let v = vec![10, 20, 30];
    let a = [1u8, 2u8];
    let mut n = 0;
    for c in "ab".chars() { n = n + c as usize; }
    println!("{} {} {}", v[1usize], v[2], a[0] + n as u8);
}
