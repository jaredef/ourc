// P-WIDTH: arithmetic on a width-typed value WRAPS to that width, as `rustc -O` does. The values come
// from functions so nothing is const-folded -- the fold already agreed; only runtime did not.
fn u8v() -> u8 { 250 }
fn i8v() -> i8 { 120 }
fn u16v() -> u16 { 65530 }
fn i32v() -> i32 { 2147483640 }
fn main() {
    let a: u8 = u8v();
    let b: u8 = 10;
    println!("{}", a + b);
    let c: i8 = i8v();
    println!("{}", c + c);
    let d: u16 = u16v();
    println!("{}", d + d);
    let e: i32 = i32v();
    println!("{}", e * 3);
    let f: u8 = 200;
    println!("{}", f as u32 + 100);
    let g: isize = 4000000000;
    println!("{}", g + g);
}
