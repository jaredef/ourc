// R2a P4 guard: a program whose own string literal is the CONST-DIV0 marker text is not a division by zero
// (the marker is a whole IR comment line; a literal lives inside a global constant).
fn main() {
    let s = "; CONSTDIV0";
    let t = "x\n; CONSTDIV0\ny";
    let d = 2;
    println!("{} {} {}", s.len(), t.len(), 10 / d);
}
