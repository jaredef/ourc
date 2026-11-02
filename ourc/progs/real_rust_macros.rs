macro_rules! two { () => (2) }
macro_rules! add { ($a:expr, $b:expr) => ($a + $b) }
macro_rules! sq { ($x:expr) => ($x * $x) }
macro_rules! pick {
    (first, $a:expr, $b:expr) => ($a);
    (second, $a:expr, $b:expr) => ($b);
}
fn main() {
    println!("{}", two!());
    println!("{}", add!(40, 2));
    println!("{}", sq!(6) + 6);
    println!("{}", pick!(first, 42, 0));
    println!("{}", pick!(second, 0, 42));
    println!("{}", add!(sq!(4), 26));
}
