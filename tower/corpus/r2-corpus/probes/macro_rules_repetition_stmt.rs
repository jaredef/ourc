// LAYER3 M1 (slice 1): macro_rules repetition -- stmt-repeat fan-out. A single repeated metavar `$($x:expr)sep*`
// with the whole body one `$( <stmts w/ $x> )*` group; emit the inner statements once per call arg, `$x` bound to
// arg k. Comma + semicolon separators, `*` + `+`, multi-statement group, single/zero elem. (The expr-accum
// embedded-group form `{ ..; $(s+=$x;)* s }` is the next slice, recorded.)
macro_rules! show {
    ($($x:expr),*) => { $( println!("v = {}", $x); )* };
}
macro_rules! show_semi {
    ($($x:expr);*) => { $( println!("s = {}", $x); )* };
}
macro_rules! doubled {
    ($($x:expr),+) => { $( let d = $x * 2; println!("{}", d); )+ };
}
fn main() {
    show!(1, 2, 3);
    show!(42);
    show!();
    show_semi!(10; 20);
    doubled!(5, 6, 7);
}
