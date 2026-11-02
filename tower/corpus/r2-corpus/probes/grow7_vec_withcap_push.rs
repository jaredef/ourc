fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(8);
    for i in 0..8 { v.push(i * i); }
    let mut s = 0;
    for x in &v { s += x; }
    println!("{}", s);
}
