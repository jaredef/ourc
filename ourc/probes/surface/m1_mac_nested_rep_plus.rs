macro_rules! m { ($( $a:ident = $($b:expr),+ );+ $(;)?) => { $( let $a: i32 = 0 $(+ $b)+; println!("{} {}", stringify!($a), $a); )+ }; }
fn main() { m!(p = 4; q = 1, 2, 3;); }
