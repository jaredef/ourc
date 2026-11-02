fn main() {
    let b: Box<i32> = Box::new(10);
    let c: Box<i32> = Box::new(32);
    let s: i32 = *b + *c;
    println!("{}", s);
}
