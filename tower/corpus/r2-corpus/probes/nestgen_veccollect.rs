fn main() {
    let g: Vec<Vec<i32>> = (0..3).map(|i| vec![i, i + 1]).collect();
    let s: i32 = g.iter().flatten().sum();
    println!("{}", s);
}
