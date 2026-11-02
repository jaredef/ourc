fn main() { let p: i64 = -9; let a: i32 = 7; let u: u32 = 17;
  println!("{} {} {} {} {} {} {} {} {} {}", p.div_euclid(4), p.signum(), a.div_euclid(-2), (-7i32).div_euclid(-2), (-7i32).div_euclid(2),
    a.div_euclid(2), 0i64.signum(), 42i8.signum(), (-3i16).signum(), u.div_euclid(5)); }
