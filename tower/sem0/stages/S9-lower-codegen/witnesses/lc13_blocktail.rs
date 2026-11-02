fn main() {
    let x: i32 = { let a: i32 = 3; let b: i32 = 4; a * b };
    println!("{}", x);
}
