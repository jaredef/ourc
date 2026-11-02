fn main() {
    let n = 7;
    let r = match n { x if x == 1 || x == 7 => x*2, _ => 0 };
    println!("{}", r);
}
