fn main() {
    let mut i = 0;
    let r = loop {
        i += 1;
        if i * i >= 50 { break i; }
    };
    println!("{}", r);
}
