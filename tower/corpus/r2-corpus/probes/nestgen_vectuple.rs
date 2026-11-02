fn main() {
    let v: Vec<(i32, i32)> = vec![(1, 2), (3, 4)];
    let s: i32 = v.iter().map(|&(a, b)| a * b).sum();
    println!("{}", s);
}
