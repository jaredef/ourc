macro_rules! two { ($($x:expr);*) => { $( println!("{}", $x); )* }; }
fn main() { two!(1; 2; 3); }
