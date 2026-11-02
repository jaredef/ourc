// RUSTC-TARGET S8 (P-ITER-MUT-FIXED-ARRAY): `.iter_mut()` over a FIXED ARRAY in a for-loop (with/without
// .enumerate()), binding the element by mutable reference -- `for (i, d) in m.iter_mut().enumerate() { *d
// = i as i32; }` (either`s `fn seek` test). The for-loop`s iter_mut handler required sym_kind==2 (Vec); a
// fixed array is sym_kind==5, so it fell through and the general parse mishandled `arr.iter_mut()`, over-
// consuming the loop body (localized by the parse-drift guard). Now the array case binds the element
// address (buffer = the array base, len = compile-time). This was either`s LAST parse-drift -- with it,
// the whole 2087-line either crate frontend-compiles.
fn main() {
    let mut m = [0; 8];
    for (i, d) in m.iter_mut().enumerate() { *d = (i as i32) * 2; }
    let mut n = [0; 4];
    for d in n.iter_mut() { *d = 5; }
    println!("{} {} {} {}", m[0], m[3], m[7], n[2]);
}
