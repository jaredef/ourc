fn main() {
    let mut last = 0;
    for (i, _c) in "hello".char_indices() { last = i; }
    println!("{}", last);
}
