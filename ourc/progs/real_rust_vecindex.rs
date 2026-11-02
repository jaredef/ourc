fn main() {
    let v = vec![10, 20, 30];
    println!("{}", v[0] + v[2]);
    let r = vec![7; 3];
    println!("{}", r[0] + r[1] + r[2] + 21);
    let mut w = vec![1, 2];
    w[1] = 41;
    println!("{}", w[0] + w[1]);
}
