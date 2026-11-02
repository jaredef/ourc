struct C;
impl C { const K: i64 = 42; fn get() -> i64 { Self::K } }
fn main() { println!("{}", C::get()); }
