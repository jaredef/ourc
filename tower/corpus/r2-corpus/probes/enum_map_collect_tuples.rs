// P-ITER-ADAPTER: .enumerate().map(|(i,&x)| (i, ..)).collect() into a Vec of tuples,
// then destructure it with `for (i,x) in &pairs` incl. `*i` deref. Requires the
// collected Vec's element tuple-type recorded (VELTY) + the destructured fields
// SFORREF-tagged (the &vec element is a ref, so *i is a no-op).
fn main() {
    let v = vec![10, 20, 30];
    let pairs: Vec<(usize, i32)> = v.iter().enumerate().map(|(i, &x)| (i, x * 2)).collect();
    let mut s = 0;
    for (i, x) in &pairs { s += (*i as i32) + x; }
    println!("{}", s);                       // (0+20)+(1+40)+(2+60) = 123
}
