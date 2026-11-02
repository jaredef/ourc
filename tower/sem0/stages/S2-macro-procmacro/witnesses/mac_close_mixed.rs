macro_rules! sq { ($x:expr) => { $x * $x }; }
macro_rules! sum { ($($x:expr),*) => { 0 $(+ $x)* }; }
#[cfg(any())]
fn dropped() {}
fn main() {
    let a = sq!(4);
    let b = sum!(1, 2, 3);
    let c = stringify!(x + y);
    let d = concat!("p", "q");
    let e = cfg!(all());
    #[cfg(any())]
    let gone = 99;
    let keep = 7;
}
