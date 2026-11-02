fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut acc = 0;
    for c in a.chunks(2) { acc = acc * 10 + c.iter().sum::<i32>(); }
    println!("{}", acc);
}
