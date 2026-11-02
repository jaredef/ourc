fn main() {
    let n: i32 = 2;
    let x: i32 = match n { 1 => 100, 2 => 200, _ => 300 };
    println!("{}", x);
}
