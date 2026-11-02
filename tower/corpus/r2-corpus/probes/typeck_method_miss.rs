// E0599 probe: calling a method that does not exist on a known struct type.
// rustc rejects (no method named `nonexistent`). The derived rzrs.rs must REJECT too (R2bf),
// rather than emit an undefined `call P_nonexistent` (link-fail / silent miscompile).
struct P { x: i32 }
fn main() {
    let p = P { x: 1 };
    p.nonexistent();
}
