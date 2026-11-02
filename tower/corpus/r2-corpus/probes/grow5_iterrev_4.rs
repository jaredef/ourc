fn main() {
    let v = vec![1, 2, 3, 4];
    let mut out = 0;
    for (i, x) in v.iter().rev().enumerate() {
        out = out * 10 + x - i as i32;
    }
    println!("{}", out);
}
