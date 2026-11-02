fn main() {
    let v = vec![1.5f64, 2.0, 0.25];
    let s: f64 = v.iter().sum();
    let p: f64 = v.iter().product();
    let m: f64 = v.iter().map(|x| x * 2.0).sum();
    let w = vec![0.1f64, 0.2];
    let t: f64 = w.iter().sum();
    println!("{} {} {} {}", s, p, m, t);
}
