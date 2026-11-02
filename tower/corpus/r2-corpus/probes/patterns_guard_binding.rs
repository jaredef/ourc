fn main() {
    let p = (3, 8);
    let r = match p { (a, b) if a < b => b - a, (a, b) => a - b };
    println!("{}", r);
}
