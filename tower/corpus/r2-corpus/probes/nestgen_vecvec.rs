fn main() {
    let mut g: Vec<Vec<i32>> = Vec::new();
    g.push(vec![1, 2, 3]);
    g.push(vec![4, 5]);
    let s: i32 = g.iter().map(|r| r.iter().sum::<i32>()).sum();
    println!("{}", s);
}
