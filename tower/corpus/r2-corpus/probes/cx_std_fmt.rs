// P-CX-SORTBY + P-CX-FMT witness (ourc construct-generation, 2026-10-24): sort_by comparators over tuple elements
// with &str components (`x.0.cmp(y.0)`, `.then(..)`), {:e} of floats (fixed-point x1e9 exponent), the `+` flag with
// width/align/zero-pad, and both in format!().
fn main() {
    let mut v = vec![("b", 3), ("a", 9), ("c", 1), ("a", 2)]; v.sort_by(|x, y| x.0.cmp(y.0).then(y.1.cmp(&x.1))); println!("{:?}", v);
    let mut w = vec![(2, 3), (1, 9), (2, 1)]; w.sort_by(|x, y| x.0.cmp(&y.0).then(y.1.cmp(&x.1))); println!("{:?}", w);
    println!("[{:e}] [{:e}] [{:e}] [{:e}] [{:e}]", 1234.5, 0.00012, 1.0, 100.0, 1234);
    println!("[{:>+6}] [{:<+6}] [{:+06}] [{:+}] [{:^+7}]", 7, 7, 7, -7, 12);
    let s = format!("[{:e}] [{:>+5}] [{:+}]", 25.5, 3, 4); println!("{} {}", s, s.len());
}
