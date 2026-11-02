// MOVEBORROW m3 conservative-strip witness: a non-move closure borrows (both uses legal); a move
// closure that is the ONLY user of the capture is legal. rustc accepts all lines here.
fn main() {
    let s = String::from("hi");
    let g = || s.len();                  // NON-move: borrows -> s still usable
    println!("{} {}", s.len(), g());
    let t = String::from("yo");
    let f = move || t.len();              // move: t captured, but only the closure uses it
    println!("{}", f());
    let v = vec![1, 2, 3];
    let h = move || v.len();
    println!("{}", h());
}
