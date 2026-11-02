// R2a P5 guard: a `static mut` declared in a module, bumped there and read / written through its path (`l::N`), with
// statement-level `unsafe { .. }` blocks followed by more statements (no `;`)
mod l {
    pub static mut N: i32 = 0;
    pub fn bump() {
        unsafe {
            N += 1;
        }
    }
}
fn main() {
    l::bump();
    l::bump();
    unsafe {
        l::N += 3;
    }
    let a = unsafe { l::N };
    println!("{}", a);
}
