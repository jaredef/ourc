// P-ITER-ADAPTER: fold with a TUPLE accumulator. Each iter destructures acc into the
// pattern names, the body yields a new tuple, copied back. 2- and 3-tuples.
fn main() {
    let v = vec![1i64, 2, 3, 4];
    let (s, p) = v.iter().fold((0i64, 1i64), |(s, p), &x| (s + x, p * x));
    println!("{}", s + p);                 // 10 + 24 = 34
    let w = vec![1, 2, 3];
    let (a, b, c) = w.iter().fold((0, 0, 0), |(a, b, c), &x| (a + x, b + 1, c + x * x));
    println!("{} {} {}", a, b, c);         // 6 3 14
    let sc = v.iter().fold(0i64, |a, &x| a + x);
    println!("{}", sc);                    // 10 (scalar unregressed)
}
