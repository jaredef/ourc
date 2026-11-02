fn main() {
    let c: i32 = 1;
    let x: i32 = if c > 0 { let a: i32 = 10; let b: i32 = 5; a + b } else { 0 };
    println!("{}", x);
}
