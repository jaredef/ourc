fn main() {
    let mut i: i32 = 0;
    let mut s: i32 = 0;
    while i < 5 {
        i = i + 1;
        s = s + i;
    }
    println!("{}", s);
}
