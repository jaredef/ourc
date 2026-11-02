// P-PAT: a pattern prefixed by `&` / `&mut` matches through a reference; our value model has no
// distinct reference, so the prefix is skipped and the pattern underneath is read as usual.
fn main() {
    let x: [isize; 3] = [1, 2, 3];
    println!("{}", match &x { &[a, b, c] => a + b + c });
    let t = (4, 5);
    println!("{}", match &t { &(p, q) => p * q });
    let v = vec![7, 8];
    for e in &v { print!("{} ", e); }
    println!();
}
