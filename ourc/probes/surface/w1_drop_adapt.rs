use std::mem;
static mut D: usize = 0;
struct F(i32);
impl Drop for F {
    fn drop(&mut self) {
        unsafe { D += 1; };
        print!("d{} ", self.0);
    }
}
fn main() {
    { let v = vec![F(1), F(2), F(3)]; let w: Vec<F> = v.into_iter().filter(|x| { print!("p{} ", x.0); x.0 != 2 }).collect(); print!("| "); }
    { let v = vec![F(4), F(5), F(6)]; let w: Vec<F> = v.into_iter().take(1).collect(); print!("| "); }
    { let v = vec![F(7), F(8), F(9)]; let w: Vec<F> = v.into_iter().skip(1).filter(|x| x.0 == 9).collect(); print!("| "); }
    { let v = vec![F(10), F(11), F(12)]; let w: Vec<F> = v.into_iter().take_while(|x| x.0 < 11).collect(); print!("| "); }
    { let mut v = vec![F(13), F(14), F(15)]; let w: Vec<F> = mem::replace(&mut v, vec![]).into_iter().skip_while(|_x| true).collect(); print!("| "); }
    { let v = vec![F(16), F(17)]; let f = |x: &F| x.0 == 17; let b = f(&v[1]); print!("b{} ", b); }
    unsafe { println!("{}", D); }
}
