macro_rules! m { ($( $a:ident : $($b:expr),* );*) => { $( let $a = 0 $(+ $b)*; println!("{}", $a); )* }; }
fn main() { m!(x: 1, 2; y: 1, 1, 1); }
