fn main() {
    let x: _ = 5;
    let v: Vec<_> = vec![1, 2];
    let f = |a: _| a + 1;
    let w = v.iter().map(|y| *y).collect::<Vec<_>>();
    println!("{} {} {}", x, f(2), w.len());
}
