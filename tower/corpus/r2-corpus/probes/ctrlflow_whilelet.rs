fn main() {
    let mut v = vec![1, 2, 3, 4];
    let mut s = 0;
    while let Some(x) = v.pop() { s += x; }
    println!("{}", s);
}
