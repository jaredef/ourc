struct Row { cells: Vec<i32> }
fn main() {
    let rows = vec![Row { cells: vec![1, 2] }, Row { cells: vec![3, 4, 5] }];
    let n: usize = rows.iter().map(|r| r.cells.len()).sum();
    println!("{}", n);
}
