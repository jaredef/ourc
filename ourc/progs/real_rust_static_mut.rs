// P-PLACE, smallest slice: a `static mut` is a real GLOBAL -- the first place in our value model. Reads
// load it, writes store to it (including through `+=`), and the initializer runs first in main (Rust
// requires it to be const-evaluable, so the order cannot be observed).
static mut COUNT: i64 = 5;
static mut FLAG: i64 = 0;
fn bump(n: i64) -> i64 { unsafe { COUNT += n; COUNT } }
fn main() {
    unsafe {
        println!("{}", COUNT);
        COUNT += 11;
        println!("{}", COUNT);
        COUNT = 100;
        println!("{} {}", COUNT, bump(3));
        FLAG = COUNT * 2;
        println!("{}", FLAG);
    }
}
