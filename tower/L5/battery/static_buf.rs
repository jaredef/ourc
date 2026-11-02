static mut BUF: [u8; 32] = [0; 32];
static mut BIG: i64 = 0;
fn main() {
    unsafe {
        let msg = "hello world";
        let mut i = 0;
        while i < 11 { BUF[i as usize] = (65 + i) as u8; i = i + 1; }
        let mut j = 0;
        while j < 11 { print!("{}", BUF[j as usize] as char); j = j + 1; }
        println!("");
        BIG = 5000000000;
        BIG = BIG + BIG;
        println!("BIG = {}", BIG);
        let mut sum: i64 = 0;
        let mut k = 0;
        while k < 11 { sum = sum + BUF[k as usize] as i64; k = k + 1; }
        println!("bytesum = {}", sum);
    }
}
