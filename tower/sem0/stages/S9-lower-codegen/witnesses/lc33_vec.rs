fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    let s: i32 = v[0] + v[2] + (v.len() as i32);
    println!("{}", s);
}
