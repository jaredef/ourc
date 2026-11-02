// while let with a Vec payload + nested + `mut` binding -- P-CONTROL-FLOW-LET rung 2:
// `while let Some(inner) = vec_of_vecs.pop()` didn't kind-tag inner as a Vec (garbage),
// and `Some(mut inner)` bound "mut" as the name. Now DVELT threads the element kind and
// `mut`/`&` are skipped.
fn main() {
    let mut stack = vec![vec![1, 2], vec![3]];
    let mut t = 0;
    while let Some(mut inner) = stack.pop() {
        while let Some(x) = inner.pop() { t += x; }
    }
    println!("{}", t);          // 6
    // single-level Vec payload -> len()
    let mut s = vec![vec![1, 2, 3], vec![4, 5]];
    let mut n = 0;
    while let Some(row) = s.pop() { n += row.len(); }
    println!("{}", n);          // 5
    // scalar mut payload unregressed
    let mut v = vec![10, 20];
    let mut sum = 0;
    while let Some(mut x) = v.pop() { x += 1; sum += x; }
    println!("{}", sum);        // 32
}
