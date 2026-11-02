// LAYER3 M1 (slice 2): macro_rules repetition -- EMBEDDED-group at expression position. The `$(...)*` group is
// inside a block `{ init; $(acc)* tail }` and the macro returns the tail value. expand_macro splits the body into
// init statements / the repeated group (emitted per call arg) / tail expr -- unified with the slice-1 fan-out
// (whole-body-is-the-group) case.
macro_rules! sum {
    ($($x:expr),*) => { { let mut s = 0; $( s += $x; )* s } };
}
macro_rules! prod {
    ($($x:expr),*) => { { let mut p = 1; $( p *= $x; )* p } };
}
macro_rules! build {
    ($($x:expr),*) => { { let mut v = Vec::new(); $( v.push($x); )* v } };
}
fn main() {
    println!("{}", sum!(1, 2, 3, 4));
    println!("{}", sum!());
    println!("{}", prod!(2, 3, 4));
    println!("{}", sum!(5, 5) + prod!(2, 5));
    let v = build!(10, 20, 30);
    println!("{:?}", v);
}
