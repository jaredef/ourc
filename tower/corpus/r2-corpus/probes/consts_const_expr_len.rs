const N: usize = 2 + 3;
fn main() {
    let a = [7i64; N];
    println!("{}", a.len() as i64 * a[0]);
}
