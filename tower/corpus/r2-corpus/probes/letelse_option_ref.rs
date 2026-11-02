// let-else with Option<&T> from .first() -- P-CONTROL-FLOW-LET rung 1:
// the refutable-let head didn't skip `&` in the payload or tag the ref-Option
// payload (EOPTREF), so `let Some(&x)=v.first() else` / `let Some(n)=..; *n` gave
// garbage/segfault. Now `&x` strips one ref and a ref-Option payload sets SFORREF.
fn head(v: &[i64]) -> i64 {
    let Some(&x) = v.first() else { return -1; };
    x * 3
}
fn head_deref(v: &[i64]) -> i64 {
    let Some(n) = v.first() else { return 0; };
    *n + 100
}
fn main() {
    let a = [7i64, 8, 9];
    let b: [i64; 0] = [];
    println!("{}", head(&a));       // 21
    println!("{}", head(&b));       // -1  (else)
    println!("{}", head_deref(&a)); // 107
    // plain Option let-else still works
    let o: Option<i32> = Some(5);
    let Some(m) = o else { return; };
    println!("{}", m * 2);          // 10
}
