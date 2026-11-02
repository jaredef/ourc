fn main(){ let a:u8=250; let b=a.wrapping_add(10); let c:i32=i32::MAX; let d=c.wrapping_add(1); println!("{} {}", b, d); }
