trait T { fn base(&self) -> i64; fn a(&self) -> i64 { self.base() + 1 } fn b(&self) -> i64 { self.a() + 1 } }
struct S;
impl T for S { fn base(&self) -> i64 { 10 } }
fn main() { println!("{}", S.b()); }
