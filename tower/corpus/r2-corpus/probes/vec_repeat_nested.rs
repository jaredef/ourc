// vec![vec![..]; n] -- regression guard for TI3e-o (N-vec-repeat-alias):
// the repeat form pushed the SAME inner-Vec descriptor n times, so all rows
// aliased (a write to g[1][j] appeared in g[0][j]). Now a Vec element is
// re-evaluated per row (fresh inner Vec each iteration).
fn main() {
    let r = 3; let c = 4;
    let mut g = vec![vec![0i32; c]; r];
    for i in 0..r { for j in 0..c { g[i][j] = (i * c + j) as i32; } }
    let mut s = 0;
    for i in 0..r { for j in 0..c { s += g[i][j]; } }
    println!("{} {}", s, g[2][3]);
    // independent rows
    let mut h = vec![vec![1i32; 2]; 2];
    h[0][0] = 9;
    println!("{} {}", h[0][0], h[1][0]);
    // scalar repeat unchanged
    let v = vec![7i32; 3];
    println!("{}", v[0] + v[1] + v[2]);
}
