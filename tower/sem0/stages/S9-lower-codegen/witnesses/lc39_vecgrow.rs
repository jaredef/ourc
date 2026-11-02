fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10); v.push(20); v.push(30); v.push(40); v.push(50); v.push(60);
    println!("{}", v[0] + v[5] + (v.len() as i32));
}
