// P-VECVEC-RESIDUAL: a &Vec<Vec<T>> param -- m[r][c] index, m[r].len() (inner row len),
// nested for over &result. Needs the param to record element-is-Vec (SVELT=-3) so m[r]
// dispatches as a Vec.
fn transpose(m: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let rows = m.len();
    let cols = m[0].len();
    let mut out: Vec<Vec<i64>> = Vec::new();
    for c in 0..cols {
        let mut row: Vec<i64> = Vec::new();
        for r in 0..rows { row.push(m[r][c]); }
        out.push(row);
    }
    out
}
fn main() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let t = transpose(&m);
    let mut acc = 0;
    for r in &t { for x in r { acc = acc * 10 + x; } }
    println!("{}", acc);                         // 142536
    println!("{}", m[0].len());                  // 3 (inner row len)
}
