fn main() {
    let a = "\"".len();
    let b = "a\tb\nc".len();
    let c = "\\".len();
    println!("{}", a * 100 + b * 10 + c);
}
