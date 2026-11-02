fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10); v.push(20); v.push(30);
    println!("len {}", v.len());
    let mut i = 0; let mut sum = 0;
    while i < v.len() { println!("v[{}] = {}", i, v[i]); sum = sum + v[i]; i = i + 1; }
    println!("sum {}", sum);
}
