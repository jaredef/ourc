fn main() {
    let v = vec![1, 2, 3, 4];
    let s: i32 = v.iter().sum();
    let p: i32 = v.iter().product();
    let q: i32 = v.iter().map(|x| x * 2).sum();
    println!("{} {} {}", s, p, q);
}
