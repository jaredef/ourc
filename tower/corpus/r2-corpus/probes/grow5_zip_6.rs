fn main() {
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];
    let c: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    println!("{}", c.iter().sum::<i32>());
}
