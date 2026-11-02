macro_rules! cnt { () => { 0 }; ($h:tt $($t:tt)*) => { 1 + cnt!($($t)*) }; } fn main() { println!("{}", cnt!(a b c d)); }
