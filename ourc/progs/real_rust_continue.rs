// C3: a `continue` inside a `for` must still advance the loop. Our desugar put the step AFTER the body,
// so `continue` jumped past it and the loop never advanced -- a HANG. The step now lives in the
// CONDITION (a `while` condition is an expression, and a sequence yields its right-hand value), so every
// path back to the top advances, including `continue`. Correct by construction rather than by a rewrite.
fn main() {
    let mut s = 0;
    for i in 0..6 { if i % 2 == 0 { continue; } s += i; }
    println!("{}", s);
    let v = vec![1, 2, 3, 4];
    let mut t = 0;
    for x in &v { if *x == 2 { continue; } t += x; }
    println!("{}", t);
    let mut u = 0;
    for a in 0..3 { for b in 0..3 { if b == 1 { continue; } u += 1; } }
    println!("{}", u);
    let mut w = vec![1, 2, 3];
    for x in w.iter_mut() { if *x == 2 { continue; } *x *= 10; }
    println!("{} {} {}", w[0], w[1], w[2]);
}
