// P-ITER-ADAPTER: for c in v.chunks_exact(n) -- non-overlapping n-slices, DROPPING the
// final partial chunk (unlike chunks). Array + Vec sources.
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7];
    let mut s = Vec::new();
    for c in a.chunks_exact(2) { s.push(c[0] + c[1]); }
    println!("{:?}", s);                     // [3, 7, 11] (7 dropped)
    let v = vec![10, 20, 30, 40, 50];
    let mut t = 0;
    for c in v.chunks_exact(3) { t += c[0] + c[1] + c[2]; }
    println!("{}", t);                       // 60 (40,50 dropped)
}
