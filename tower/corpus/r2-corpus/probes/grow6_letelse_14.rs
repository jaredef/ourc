fn first(v: &[i64]) -> i64 {
    let Some(&x) = v.first() else { return -1; };
    x * 3
}
fn main() {
    let a = [7i64, 8, 9];
    let b: [i64; 0] = [];
    println!("{}", first(&a) + first(&b));
}
