macro_rules! getter { ($n:ident, $v:expr) => { fn $n() -> i64 { $v } }; } getter!(answer, 42); fn main() { println!("{}", answer()); }
