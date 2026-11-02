// P-TYPE-RAW-POINTER: `*mut T` / `*const T` raw-pointer params parse; the pointer is a scalar machine
// pointer so `*p` does a real deref (unlike a value-transparent `&scalar`). rustc Vec::from_raw_parts.
fn load_mut(p: *mut i32) -> i32 { unsafe { *p } }
fn load_const(p: *const i32, off: usize) -> i32 { unsafe { *p + (off as i32) } }
fn main() {
    let mut a = 7;
    let b = 35;
    println!("{} {}", load_mut(&mut a), load_const(&b, 7));
}
