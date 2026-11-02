fn main() { let z = 0.0_f64 / 0.0; let i = 1.0_f64 / 0.0;
  println!("{} {} {} {} {} {} {}", z.is_nan(), i.is_infinite(), i.is_finite(), f64::NAN == f64::NAN, f64::NAN, f64::INFINITY, f64::NEG_INFINITY);
  println!("{} {} {} {}", (-2.5_f64).signum(), 0.0_f64.signum(), (-0.0_f64).signum(), f64::EPSILON); }
