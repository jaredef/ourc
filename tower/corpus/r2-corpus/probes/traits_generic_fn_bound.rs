trait Val { fn val(&self) -> i64; }
struct S(i64);
impl Val for S { fn val(&self) -> i64 { self.0 } }
fn get<T: Val>(t: T) -> i64 { t.val() }
fn main() { println!("{}", get(S(42))); }
