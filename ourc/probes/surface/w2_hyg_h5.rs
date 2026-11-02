macro_rules! pick { ($o:expr) => { match $o { Some(v) => v, None => 0 } } } fn main(){ let v = 7; println!("{} {}", pick!(Some(5)), pick!(Some(v))); }
