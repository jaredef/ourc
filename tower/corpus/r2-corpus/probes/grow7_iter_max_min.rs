fn main() {
    let v = vec![7, 2, 9, 4, 1, 8];
    let mx = v.iter().max().unwrap();
    let mn = v.iter().min().unwrap();
    println!("{} {}", mx, mn);
}
