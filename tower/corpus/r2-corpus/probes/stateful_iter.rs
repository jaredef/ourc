// STATEFUL-ITER: a bound `let mut it = v.iter()` is a cursor -- it.next() advances,
// it.peek() looks ahead; while let Some(&x)=it.next() integrates; Option== is structural
// (it.peek()==Some(&&x)); (range).peekable() still eager-dispatches sum/etc.
fn main() {
    let v = vec![1, 2, 3, 4];
    let mut it = v.iter();
    let mut s = 0;
    while let Some(&x) = it.next() { s += x; }
    println!("{}", s);                           // 10
    // peek + dedup groups
    let w = vec![1, 1, 2, 3, 3];
    let mut it2 = w.iter().peekable();
    let mut groups = 0;
    while let Some(&x) = it2.next() {
        groups += 1;
        while it2.peek() == Some(&&x) { it2.next(); }
    }
    println!("{}", groups);                      // 3
    // eager fallthrough on a peekable range
    let mut it3 = (1..5).peekable();
    let p = *it3.peek().unwrap();
    let sm: i32 = it3.sum();
    println!("{}", p + sm);                      // 1 + 10 = 11
}
