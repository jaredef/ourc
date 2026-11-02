fn main() {
    let v: Vec<(i32, i32)> = vec![(1, 2), (3, 4), (5, 6)];
    let mut s = 0;
    for (a, b) in &v { s += a * b; }
    println!("{}", s);
}
