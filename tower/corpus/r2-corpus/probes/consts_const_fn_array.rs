const fn dbl(x: i64) -> i64 { x + x }
const A: [i64; 3] = [dbl(1), dbl(2), dbl(3)];
fn main() { println!("{}", A[0] + A[1] + A[2]); }
