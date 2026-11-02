// P-PLACE, probed: a `&mut` to an AGGREGATE needs nothing (our handles already behave as references),
// `iter_mut` is the same write-back loop as `for x in &mut v`, and READING through a `&mut` scalar is
// identical by value. Only WRITING through a `&mut` scalar needs a place -- and that is refused.
struct P { x: i32 }
fn push2(v: &mut Vec<i32>) { v.push(2); }
fn bump(p: &mut P) { p.x += 1; }
fn double(x: &mut i32) -> i32 { *x * 2 }
fn get(x: &i32) -> i32 { *x }
fn main() {
    let mut v = vec![1];
    push2(&mut v);
    let mut p = P { x: 1 };
    bump(&mut p);
    let mut n = 4;
    println!("{} {} {} {} {}", v.len(), v[1], p.x, double(&mut n), get(&n));
    let mut w = vec![1, 2, 3];
    for x in w.iter_mut() { *x *= 10; }
    println!("{} {} {}", w[0], w[1], w[2]);
}
