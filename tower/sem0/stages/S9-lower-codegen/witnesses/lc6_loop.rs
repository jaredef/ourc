fn main() {
    let mut i: i32 = 0;
    let x: i32 = loop {
        i = i + 1;
        if i == 5 { break i * 10; }
    };
    println!("{}", x);
}
