fn main() {
    let mut acc = 0;
    for i in 0..5 { if i % 2 == 0 { acc += i; } }
    println!("{}", acc);
}
