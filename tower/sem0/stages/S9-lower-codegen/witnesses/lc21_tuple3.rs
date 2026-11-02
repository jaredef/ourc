fn main() {
    let t: (i32, i32, i32) = (10, 20, 30);
    let s: i32 = t.0 * t.2 - t.1;
    println!("{}", s);
}
