fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    let x = v.pop().unwrap();
    println!("{}", x + v.len() as i64);
}
