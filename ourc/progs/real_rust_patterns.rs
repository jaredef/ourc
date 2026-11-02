// P-PAT stage 2: structural MATCH patterns -- tuple, struct, slice (with literal positions, a rest, and
// positions addressed from the END), integer ranges, `ref` bindings, and `move` closures.
struct P { x: isize, y: isize }
fn grade(n: isize) -> isize { match n { 0..=9 => 1, 10..=99 => 2, _ => 3 } }
fn shape(a: [isize; 3]) -> isize {
    match a { [1, 2, 4] => 10, [0, .., 3] => 20, [1, 2, 3] => 30, [_, _, _] => 40 }
}
fn main() {
    let t = (4, 5);
    println!("{}", match t { (a, b) => a * b });
    let p = P { x: 2, y: 7 };
    println!("{}", match p { P { x, y } => x + y });
    println!("{} {} {}", grade(5), grade(42), grade(900));
    println!("{} {}", shape([1, 2, 3]), shape([0, 9, 3]));
    let o = Some(3);
    match o { Some(ref v) => println!("v={}", v), None => println!("none") }
    let k = 6;
    let f = move |z: isize| z * k;
    println!("{}", f(7));
}
