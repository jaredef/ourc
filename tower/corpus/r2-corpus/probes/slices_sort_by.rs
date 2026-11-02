fn main() {
    let mut a = [4, 2, 5, 1, 3];
    a.sort_by(|x, y| y.cmp(x));
    let mut acc = 0;
    for x in a { acc = acc * 10 + x; }
    println!("{}", acc);
}
