fn exp(x: i64) -> i64 { x * 2 }
fn main() { let x = 2.0_f64; let y = 0.5_f64;
  println!("{} {} {} {}", x.ln(), x.exp(), x.exp2(), 10.0_f64.log10());
  println!("{} {} {} {}", y.sin(), y.cos(), 8.0_f64.log2(), x.powf(0.5));
  println!("{} {} {}", 2.0_f64.powf(10.0), (-1.0_f64).ln(), 0.0_f64.ln());
  println!("{:?} {}", 1.0_f64.exp(), exp(4)); }
