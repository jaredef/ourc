trait Val { fn val(&self) -> i64; }
struct S(i64);
impl Val for S { fn val(&self) -> i64 { self.0 } }
fn dbl<T>(t: T) -> i64 where T: Val { t.val() * 2 }
fn main() { println!("{}", dbl(S(7))); }
