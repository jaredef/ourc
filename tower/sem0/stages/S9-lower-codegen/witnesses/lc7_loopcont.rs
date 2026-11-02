fn main() {
    let mut i: i32 = 0;
    let mut s: i32 = 0;
    loop {
        i = i + 1;
        if i > 10 { break; }
        if i == 3 { continue; }
        s = s + i;
    }
    println!("{}", s);
}
