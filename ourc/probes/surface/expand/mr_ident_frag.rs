macro_rules! mk { ($n:ident, $v:expr) => { let $n = $v; }; } fn main() { mk!(z, 41); println!("{}", z + 1); }
