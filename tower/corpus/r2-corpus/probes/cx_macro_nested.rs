// L2 MACRORX-NESTED-BIND: a macro_rules matcher whose bracket group holds its own repetition, recursing by
// tt-munching (rev!), plus a literal-group prefix before a repetition (pairs!).
macro_rules! rev { ([] $($r:expr),*) => { vec![$($r),*] }; ([$h:expr $(, $t:expr)*] $($r:expr),*) => { rev!([$($t),*] $h $(, $r)*) }; }
macro_rules! pairs { ([$($k:expr),*] [$($v:expr),*]) => { vec![$(($k, $v)),*] }; }
fn main() {
    let a: Vec<i32> = rev!([1, 2, 3, 4]);
    println!("{:?}", a);
    let b: Vec<i32> = rev!([7]);
    println!("{:?}", b);
    let p: Vec<(i32, i32)> = pairs!([1, 2, 3] [10, 20, 30]);
    let mut s = 0; for (k, v) in p.iter() { s = s + k * v; }
    println!("{} {}", p.len(), s);
}
