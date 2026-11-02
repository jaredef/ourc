fn main() {
    let v = vec![9, 8, 7];
    for (i, x) in v.iter().enumerate() { println!("{} {}", i, x); }
    let t = vec![(1, 2), (3, 4)];
    for (a, b) in t { println!("{}", a + b); }
}
