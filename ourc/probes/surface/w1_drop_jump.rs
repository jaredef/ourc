static mut D: usize = 0;
struct F(i32);
impl Drop for F {
    fn drop(&mut self) {
        unsafe { D += 1; };
        print!("d{} ", self.0);
    }
}
fn main() {
    { let v = vec![F(1), F(2), F(3)]; for x in v { continue; } }
    let mut i = 0;
    while i < 3 { let a = F(i); i += 1; if i == 2 { continue; } let b = F(10 + i); }
    'o: for i in 0..2 { let a = F(20 + i); for j in 0..2 { let b = F(30 + j); if j == 1 { continue 'o; } } }
    let mut k = 0;
    while k < 3 { let a = F(40 + k); k += 1; if k == 2 { break; } }
    'p: loop { let a = F(50); loop { let b = F(51); break 'p; } }
    unsafe { println!("{}", D); }
}
