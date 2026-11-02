macro_rules! total { ($($x:expr),*) => (0 $(+ $x)*) }
macro_rules! myvec { ($($x:expr),*) => ({ let v = vec![$($x),*]; v }) }
macro_rules! add { ($a:expr, $b:expr) => ($a + $b) }
fn main() {
    println!("{}", total!(10, 14, 18));
    println!("{}", total!());
    println!("{}", total!(42));
    let v = myvec!(1, 2, 39);
    println!("{}", v[0] + v[1] + v[2]);
    println!("{}", add!(total!(20, 20), 2));
}
