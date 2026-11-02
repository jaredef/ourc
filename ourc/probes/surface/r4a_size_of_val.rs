use std::mem;
struct Q { a: u8, b: u32, c: u16 }
fn main() {
    let x = 7u32;
    let q = Q { a: 1, b: 2, c: 3 };
    println!("{} {} {} {}", mem::size_of_val(&[0u16; 5]), mem::size_of_val(&x), mem::size_of_val(&q), q.a as u32 + q.b + q.c as u32);
}
