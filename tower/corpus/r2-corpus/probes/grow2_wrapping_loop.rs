fn main(){ let mut x=250u8; for _ in 0..10 { x=x.wrapping_add(1); } println!("{}", x); }
