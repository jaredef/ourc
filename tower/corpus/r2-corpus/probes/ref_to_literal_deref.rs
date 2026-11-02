// D004 micro-edge: `&<int-literal>` is a ref to an rvalue temporary. cruft represents a scalar `&prim`
// by value (the primitive-by-value convention), so a direct deref must be a no-op (r IS the value),
// matching rustc's `*r == 7`. Was: `let r = &7; *r` did a real eload(7) -> crash.
fn main() {
    let r = &7;
    let a = *r;                                   // direct ref-to-literal deref
    let b = *&5 + 1;                              // deref an inline &literal
    let o: Option<&i32> = Some(&41);
    let c = match o { Some(e) => *e, None => 0 };  // by-value match of Some(&literal)
    println!("{} {} {}", a, b, c);
}
