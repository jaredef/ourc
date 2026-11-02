macro_rules! mk { ($n:ident, $t:ty) => { struct $n { f: $t } } }
macro_rules! mkfn { ($n:ident) => { fn $n() -> i32 { 0 } } }
mk!(Sname, u32);
mkfn!(gen_fn);
fn main() {}
