fn main() {
    let r = 3; let c = 4;
    let mut g = vec![vec![0i32; c]; r];
    for i in 0..r {
        for j in 0..c {
            g[i][j] = (i * c + j) as i32;
        }
    }
    let mut s = 0;
    for i in 0..r { for j in 0..c { s += g[i][j]; } }
    println!("{} {}", s, g[2][3]);
}
