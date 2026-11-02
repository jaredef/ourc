const N: usize = 3; fn main(){ let mut a = [1u8; N]; a[2] = 9; let s: u32 = a.iter().map(|&x| x as u32).sum(); println!("{:?} {}", a, s); }
