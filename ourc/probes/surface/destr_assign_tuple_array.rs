fn main() { let mut a = 1; let mut b = 2; (a, b) = (b, a); let mut c = 0; let mut d = 0; (c, _) = (3, 4); [d, _] = [7, 8];
  let mut s = String::new(); let mut n = 0; (s, n) = (String::from("hi"), a + b); println!("{} {} {} {} {} {}", a, b, c, d, s, n); }
