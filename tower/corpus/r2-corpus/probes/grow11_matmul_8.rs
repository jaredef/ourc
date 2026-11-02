fn main() {
    let a = vec![vec![1i64, 2], vec![3, 4]];
    let b = vec![vec![5i64, 6], vec![7, 8]];
    let mut c = vec![vec![0i64, 0], vec![0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            let mut s = 0i64;
            for k in 0..2 {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s;
        }
    }
    println!("{} {} {} {}", c[0][0], c[0][1], c[1][0], c[1][1]);
}
