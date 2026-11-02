fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let r: Vec<i32> = v.iter().rev().cloned().collect();
    println!("{} {} {}", r[0], r[2], r[4]);
}
