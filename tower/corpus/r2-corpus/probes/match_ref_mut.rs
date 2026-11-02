// ref / ref mut binding in a match arm -- P-MATCH-PATTERN rung 5 (the place-semantics one):
// the arm bound x to the scrutinee VALUE, not its place, and `*x += 10` in the arm body
// wasn't recognized as a deref-assign statement. Now: `ref[ mut] x` binds x to the
// scrutinee's slot address (eref, like `&mut n`), and a `=> *p [op]= e` arm body stores.
fn main() {
    let mut n = 5;
    match n { ref mut x => *x += 10 }
    println!("{}", n);              // 15
    let m = 42;
    match m { ref x => println!("{}", *x) }  // 42 (immutable ref, read)
    let mut c = 3;
    match c { ref mut x => *x *= 4 }
    println!("{}", c);              // 12
    // deref-VALUE arm (no assign) must still work
    let v = vec![7, 8];
    let r = match v.first() { Some(x) => *x, None => 0 };
    println!("{}", r);             // 7
    // plain binding arm unregressed
    println!("{}", match n { z => z + 1 });  // 16
}
