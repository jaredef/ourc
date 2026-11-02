fn main() {
    let grid: Vec<Vec<i32>> = (0..3).map(|i| (0..3).map(|j| i*3+j).collect()).collect();
    let mut s = 0;
    for row in &grid { for &x in row { s += x; } }
    println!("{}", s);
}
