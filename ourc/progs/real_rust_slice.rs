// a SLICE parameter takes a vector by reference; an array literal bound through a slice annotation keeps
// the array representation. Both must index correctly (P-PATH/P-TYPE: `[T]` names no representation).
fn first(x: &[isize]) -> isize { x[0] }
fn third(x: &[isize]) -> isize { x[2] }
const AA: [isize; 3] = [4, 5, 6];
fn main() {
    let p = vec![7, 8, 9];
    let b: &[isize] = &[1, 2, 3];
    println!("{}", first(&p));
    println!("{}", third(&p));
    println!("{}", b[1]);
    println!("{}", AA[2]);
    let n = None::<String>;
    match n { Some(_s) => println!("some"), None => println!("none") }
}
