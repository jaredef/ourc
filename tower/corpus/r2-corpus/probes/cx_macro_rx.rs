// P-CX-MACRORX / ITER-PARAM / IFLET-CAP witness (ourc construct-generation, 2026-10-25): recursive macro_rules --
// tt-munching `($h:tt $($t:tt)*)` (prefix ends in a fragment, no separator) and a comma-recursive `$x:expr` arm that
// must NOT swallow `1 , 2`; a `&mut Peekable<vec::IntoIter<String>>` param receives the caller`s cursor; a closure
// capturing an `if let` binder compares Strings.
macro_rules! count { () => { 0usize }; ($h:tt $($t:tt)*) => { 1usize + count!($($t)*) }; }
macro_rules! maxm { ($x:expr) => { $x }; ($x:expr, $($y:expr),+) => { { let a = $x; let b = maxm!($($y),+); if a > b { a } else { b } } }; }
macro_rules! tail { ($h:tt $($t:tt)*) => { vec![$($t),*] }; }
#[derive(Debug, Clone, PartialEq)] enum T { Tok(String), N(i64) }
fn go(it: &mut std::iter::Peekable<std::vec::IntoIter<String>>) -> usize { let mut n = 0; while let Some(t) = it.next() { if it.peek().is_some() { n += t.len(); } } n }
fn main() {
    println!("count={} {} {}", count!(), count!(a), count!(a b (c d) [e]));
    println!("max={} {}", maxm!(3, 9, 2, 7), maxm!(5));
    let v: Vec<i32> = tail!(1 2 3); println!("{:?}", v);
    let w: Vec<String> = "ab cde f".split(' ').map(|s| s.to_string()).collect(); println!("{}", go(&mut w.into_iter().peekable()));
    let names = vec!["dbl".to_string(), "quad".to_string()]; let ts = vec![T::Tok("quad".into()), T::N(3)];
    if let T::Tok(name) = &ts[0] { println!("{:?}", names.iter().position(|r| r == name)); }
}
