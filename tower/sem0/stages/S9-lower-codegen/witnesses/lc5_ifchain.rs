fn main() {
    let n: i32 = 2;
    let x: i32 = if n == 1 { 100 } else if n == 2 { 200 } else { 300 };
    println!("{}", x);
}
