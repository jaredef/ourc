fn main() {
    let a = [3, 7, 1, 9, 4];
    let mx = *a.iter().max().unwrap();
    let mn = *a.iter().min().unwrap();
    println!("{}", mx * 10 + mn);
}
