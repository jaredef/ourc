fn main() {
    let g: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    let mut s = 0;
    for row in &g { for x in row { s += x; } }
    println!("{}", s + g.len() as i32);
}
