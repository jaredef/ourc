struct W(isize, isize);
struct N(isize);
fn compute(i: W) -> isize { let W(a, b) = i; a * b }
fn main() {
    println!("{}", compute(W(6, 7)));
    let n = N(9);
    let N(v) = n;
    println!("{}", v);
    let m = N(4);
    println!("{}", match m { N(ref z) => z + 1 });
}
