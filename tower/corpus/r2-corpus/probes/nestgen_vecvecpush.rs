fn main() {
    let mut g: Vec<Vec<i32>> = Vec::new();
    for i in 0..3 {
        let mut row = Vec::new();
        for j in 0..3 { row.push(i * j); }
        g.push(row);
    }
    println!("{}", g[2][2]);
}
