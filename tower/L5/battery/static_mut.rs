static mut COUNT: i32 = 0;
static mut TABLE: [i32; 16] = [0; 16];
static mut TOTAL: i32 = 100;
fn bump() { unsafe { COUNT = COUNT + 1; } }
fn main() {
    unsafe {
        let mut i = 0;
        while i < 16 { TABLE[i as usize] = (i * i) as i32; i = i + 1; }
        let mut s = 0;
        i = 0;
        while i < 16 { s = s + TABLE[i as usize]; i = i + 1; }
        println!("sum of squares 0..16 = {}", s);
        bump(); bump(); bump();
        println!("count = {}", COUNT);
        TOTAL = TOTAL + s;
        println!("total = {}", TOTAL);
        TABLE[3] = 999;
        println!("TABLE[3] = {}", TABLE[3]);
    }
}
