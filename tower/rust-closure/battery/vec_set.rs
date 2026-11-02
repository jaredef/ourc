fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    while i < v.len() { v[i] = v[i] * 10; i = i + 1; }
    let mut i2 = 0;
    while i2 < v.len() { println!("{}", v[i2]); i2 = i2 + 1; }
}
