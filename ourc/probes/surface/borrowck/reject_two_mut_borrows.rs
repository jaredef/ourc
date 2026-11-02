fn main() { let mut a = 1; let r1 = &mut a; let r2 = &mut a; *r1 += 1; *r2 += 1; }
