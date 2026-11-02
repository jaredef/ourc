// LAYER4 leaf-crate frontier: u8/char `.is_ascii()` -> bool (v <= 127). rustc_lexer's unescape uses
// `if n.is_ascii() { .. } else { .. }` (MixedUnit::from). Previously unrecognized -> the parser mis-consumed
// the following `{` when is_ascii() was an if-EXPRESSION condition (parse drift). Now a clean bool method.
// (The enum-valued if-EXPRESSION-as-fn-return is a SEPARATE pre-existing emit gap -- NOELF on the prior tip
// too -- recorded, not this rung.)
fn classify(n: u8) -> i32 {
    if n.is_ascii() {
        1
    } else {
        0
    }
}
fn main() {
    let a: u8 = 65;
    let b: u8 = 200;
    println!("{} {}", a.is_ascii(), b.is_ascii());
    println!("{} {}", classify(a), classify(b));
    let c = 'z';
    println!("{}", c.is_ascii());
    let n: u8 = 100;
    println!("{}", n.is_ascii() && n > 50);
    let hi: u8 = 128;
    println!("{}", hi.is_ascii());
}
