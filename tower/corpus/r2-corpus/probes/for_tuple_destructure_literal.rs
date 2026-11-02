// for (a,b) in <array/vec literal of tuples> -- regression guard for TI3e-f:
// a fresh vec!/array literal iterable carries its element Ty in VELTY/EARRVTY,
// not DVELTY; thread it so the tuple-destructure fires instead of the enumerate
// (index,value) fallback that produced garbage.
fn main() {
    let mut s = 0;
    for (a, b) in [(1, 2), (3, 4), (5, 6)] { s += a * 10 + b; }
    println!("{}", s);
    let mut t = 0;
    for (a, b) in vec![(10, 1), (20, 2)] { t += a + b; }
    println!("{}", t);
    // by-ref bound tuple array still works (no regression path)
    let v = [(7, 8), (9, 1)];
    for (a, b) in &v { print!("{},", a + b); }
    println!();
}
