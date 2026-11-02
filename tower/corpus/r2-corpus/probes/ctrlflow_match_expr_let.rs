fn main() {
    let n = 3;
    let s = match n { 1 => 10, 2 => 20, 3 => 30, _ => 0 };
    println!("{}", s);
}
