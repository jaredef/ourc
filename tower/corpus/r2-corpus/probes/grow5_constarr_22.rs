const T: [i32; 4] = [3, 1, 4, 1];
fn main() {
    let mut s = 0;
    for i in 0..4 { s += T[i]; }
    println!("{}", s);
}
