fn main() {
    let n = "  foo   bar baz ".split_whitespace().count();
    println!("{}", n);
}
