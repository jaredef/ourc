fn total(s: &[i32]) -> i32 {
    let mut t = 0;
    for &x in s { t += x; }
    t
}
fn main() {
    let a = [10, 20, 30];
    println!("{}", total(&a));
}
