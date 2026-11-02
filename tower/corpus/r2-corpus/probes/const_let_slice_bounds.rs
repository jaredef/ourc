// CONSTEVAL R3 witnesses: const-let-propagation must fire ONLY on a provably-OOB static index.
// In-range const/literal index, a RUNTIME loop index, a `mut` (reassignable) index, and a
// const-arithmetic in-range index must ALL still compile+run. rustc accepts all of these.
fn main() {
    let a = [10, 20, 30, 40];
    let i = 2;             // const-known
    let n = 4;
    let j = n - 1;         // const-prop through arithmetic (=3, in range)
    let mut k = 9; k = 0;  // mut -> not const-tracked
    let mut s = 0;
    for t in 0..4 { s += a[t]; }   // runtime index
    println!("{}|{}|{}|{}", a[i], a[j], a[k], s);
}
