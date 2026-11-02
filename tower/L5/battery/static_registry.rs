/* a mini symbol-table pattern -- exactly the shape rzrs uses (parallel global arrays + a count) */
static mut NAMES: [i32; 64] = [0; 64];
static mut VALS: [i32; 64] = [0; 64];
static mut N: i32 = 0;
fn add(name: i32, val: i32) { unsafe { NAMES[N as usize] = name; VALS[N as usize] = val; N = N + 1; } }
fn find(name: i32) -> i32 {
    unsafe {
        let mut i = 0;
        while i < N { if NAMES[i as usize] == name { return VALS[i as usize]; } i = i + 1; }
        -1
    }
}
fn main() {
    add(10, 100); add(20, 200); add(30, 300);
    unsafe { println!("count {}", N); }
    println!("find 20 = {}", find(20));
    println!("find 99 = {}", find(99));
    println!("find 30 = {}", find(30));
}
