fn sum(v: &Vec<i32>) -> i32 {
    let mut i = 0; let mut s = 0;
    while i < v.len() { s = s + v[i]; i = i + 1; }
    s
}
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("sum {} len {}", sum(&v), v.len());
}
