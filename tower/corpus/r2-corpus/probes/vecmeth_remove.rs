fn main() {
    let mut v = vec![10, 20, 30];
    let r = v.remove(1);
    println!("{}", r + v.len() as i64);
}
