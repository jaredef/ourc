fn main() { let x = 0xF0u8 & 0x3C; let a: u16 = 0x00FF; let b = a ^ 0xFFFF; let c: i8 = -1; let d = c & 0x7F; let y = 200u8 | 1;
  println!("{} {} {} {} {} {} {}", x, !x, x.leading_zeros(), b.count_ones(), d.count_ones(), y.wrapping_add(100), (0b1010_u8 | 0x0F).count_ones()); }
