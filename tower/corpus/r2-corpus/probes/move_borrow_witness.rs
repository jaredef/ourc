// MOVEBORROW m1 conservative-strip witness: all borrow-positions + reassignment stay legal
// (rustc accepts every line here); the REJECT case is the separate borrow_use_after_move corpus probe.
fn main() {
    let s = String::from("hi");
    println!("{}", s); println!("{}", s);   // Display arg is a borrow (the struct checker's blind spot)
    println!("{}", s.len());                 // method borrow
    let r = &s; println!("{}", r);           // & borrow
    let t = String::from("hi");
    println!("{}", s == t);                  // == borrow
    let mut a = String::from("x");
    let b = a;                               // MOVE
    a = String::from("y");                   // reassign -> un-moves a
    let mut v = vec![1, 2];
    v.push(3);
    for x in &v { let _ = x; }               // Vec borrow
    println!("{} {} {}", a, b, v.len());
}
