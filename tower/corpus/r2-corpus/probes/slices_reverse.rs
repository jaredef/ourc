fn main() {
    let mut a = [1, 2, 3, 4];
    a.reverse();
    let mut acc = 0;
    for x in a { acc = acc * 10 + x; }
    println!("{}", acc);
}
