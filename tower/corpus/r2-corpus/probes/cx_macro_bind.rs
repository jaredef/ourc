// L2 rung C: macro arms bound by the recursive matcher. Shapes from rustc_* sources: a tt that is a whole group
// (count!), a ty fragment ending a separated repetition (casts!), a `$($msg:tt)*` tail that keeps its commas
// (fmt_tail!), a `::`-separated ident repetition (path_len!), and nested repetitions (grid!).
macro_rules! count { () => { 0usize }; ($h:tt $($t:tt)*) => { 1usize + count!($($t)*) }; }
macro_rules! casts { ($base:expr, $($name:ident : $ty:ty),*) => { vec![$(($base as $ty) as i64 + 0 * $name),*] }; }
macro_rules! fmt_tail { ($n:expr, $($msg:tt)*) => { format!("{}: {}", $n, format!($($msg)*)) }; }
macro_rules! path_len { ($($part:ident)::*) => { [$(stringify!($part)),*].len() }; }
macro_rules! grid { ($($($v:expr),* ;)*) => { vec![$(vec![$($v),*]),*] }; }
fn main() {
    println!("{}", count!((a b) [c] {d e f} g));
    let (x, y, z) = (1i64, 2i64, 3i64);
    let c: Vec<i64> = casts!(200, x: i64, y: u8, z: i16);
    println!("{:?}", c);
    println!("{}", fmt_tail!(3, "{}-{}", 4, 5));
    println!("{}", path_len!(core::marker::Copy));
    let g: Vec<Vec<i32>> = grid!(1, 2; 3; 4, 5, 6;);
    println!("{:?}", g);
}
