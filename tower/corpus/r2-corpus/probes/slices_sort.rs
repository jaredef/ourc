fn main() {
    let mut a = [4, 2, 5, 1, 3];
    a.sort();
    let mut acc = 0;
    for x in a { acc = acc * 10 + x; }
    println!("{}", acc);
}
