macro_rules! m { ($e:expr) => {{ match Some(9) { Some(x) => $e + x, None => 0 } }} } fn main(){ let x = 1; println!("{}", m!(x)); }
