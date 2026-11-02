// P-VECVEC-RESIDUAL: Vec<Vec<_>> built via Vec::new()+push, iterated with a nested
// inner .iter().sum() through a map, summed. Needs: the annotation records element-is-Vec
// (SVELT=-3), the map result element kind resets to the body's (scalar, not -3), and the
// sum::<T>() turbofish is erased so the inner sum routes to the builtin.
fn main() {
    let mut g: Vec<Vec<i32>> = Vec::new();
    g.push(vec![1, 2, 3]);
    g.push(vec![4, 5]);
    let s: i32 = g.iter().map(|r| r.iter().sum::<i32>()).sum();
    println!("{}", s);                           // 6 + 9 = 15
    // literal source too
    let h: Vec<Vec<i32>> = vec![vec![10, 20], vec![30]];
    let t: i32 = h.iter().map(|r| r.iter().sum::<i32>()).sum();
    println!("{}", t);                           // 30 + 30 = 60
}
