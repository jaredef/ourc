fn main() {
    let r = 1..4;
    let s: i32 = r.sum();
    let q = 1..=3;
    let t: i32 = q.sum();
    let h = 2..6;
    let n: i32 = 4;
    let w = 0..n;
    println!("{} {} {} {} {}", s, t, h.len(), h.contains(&5), w.len());
    for i in 1..=2 { print!("{} ", i); }
    println!();
}
