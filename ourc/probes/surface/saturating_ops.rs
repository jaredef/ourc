fn main() { let d: u32 = 7; let e: i16 = 32767; let b: u8 = 250; let c: i8 = -100;
  println!("{} {} {} {} {} {} {} {} {}", d.saturating_sub(9), d.saturating_add(u32::MAX), e.saturating_add(5), e.saturating_mul(2),
    b.saturating_add(10), b.saturating_mul(2), c.saturating_sub(100), c.saturating_add(-50), c.saturating_mul(2)); }
