static T: [i64; 4] = [3, 5, 7, 9];
fn main() {
    let mut s = 0i64;
    let mut i = 0; while i < 4 { s += T[i]; i += 1; }
    println!("{}", s);
}
