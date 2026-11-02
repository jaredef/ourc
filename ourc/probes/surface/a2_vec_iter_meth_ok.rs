fn main() {
    let v = vec![1, 2, 3];
    let s: i32 = v.iter().map(|x| x * 2).sum();
    let w: Vec<i32> = v.iter().filter(|x| **x > 1).cloned().collect();
    let v = v.iter();
    let c = v.map(|x| x + 1).count();
    println!("{} {} {}", s, w.len(), c);
}
