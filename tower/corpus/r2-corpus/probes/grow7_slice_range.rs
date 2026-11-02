fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let sl = &v[1..4];
    let mut s = 0;
    for x in sl { s += x; }
    println!("{} {}", sl.len(), s);
}
