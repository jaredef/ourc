fn main() {
    let s = "   42   ";
    let n: i32 = s.trim().parse().unwrap();
    println!("{}", n + 1);
}
