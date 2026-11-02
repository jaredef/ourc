fn main() {
    let data = [200u8, 100, 50, 250];
    let mut sum: u8 = 0;
    for &b in data.iter() { sum = sum.wrapping_add(b); }
    println!("{}", sum);
}
