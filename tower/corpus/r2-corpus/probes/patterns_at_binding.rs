fn main() {
    let n = 5;
    let r = match n { x @ 1..=5 => x*10, _ => 0 };
    println!("{}", r);
}
