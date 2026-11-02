// CONSTEVAL R1 witness: in-range const index, const arithmetic over elems, and a RUNTIME
// index (must NOT be treated as a const OOB) all still compile+run. rustc accepts all.
const A: [i64; 4] = [10, 20, 30, 40];
const X: i64 = A[2];
const S: i64 = A[0] + A[3];
fn main() {
    let i = 1;
    println!("{}|{}|{}", X, S, A[i]);
}
