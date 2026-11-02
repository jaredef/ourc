fn main() {
    let n: i32 = 9;
    let x: i32 = match n { 1 => 10, 2 => 20, _ => 99 };
    println!("{}", x);
}
