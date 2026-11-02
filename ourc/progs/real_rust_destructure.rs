struct P { x: isize, y: isize }
fn main() {
    let (a, b) = (3, 4);
    println!("{}", a + b);
    let p = P { x: 10, y: 20 };
    let P { x, y } = p;
    println!("{}", x + y);
    let t = (1, 2, 3);
    let (_, m, _) = t;
    println!("{}", m);
}
