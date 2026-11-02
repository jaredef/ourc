fn main() {
    let mut i = 0;
    let mut j = 10;
    let mut steps = 0;
    while i < 5 && j > 5 {
        i += 1;
        j -= 1;
        steps += 1;
    }
    println!("{}", steps * 100 + i * 10 + j);
}
