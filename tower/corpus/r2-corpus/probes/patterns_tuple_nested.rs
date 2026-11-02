fn main() {
    let t = ((1, 2), (3, 4));
    let ((a, _), (_, d)) = t;
    println!("{}", a + d);
}
