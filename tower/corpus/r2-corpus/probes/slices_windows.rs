fn main() {
    let a = [1, 2, 3, 4];
    let mut acc = 0;
    for w in a.windows(2) { acc += w[0] * w[1]; }
    println!("{}", acc);
}
