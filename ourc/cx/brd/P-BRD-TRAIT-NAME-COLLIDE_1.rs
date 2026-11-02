struct S(i32);
impl S { fn val(&self) -> i32 { self.0 } }
trait T1 { fn val(&self) -> i32; }
trait T2 { fn val(&self) -> i32; }
impl T1 for S { fn val(&self) -> i32 { self.0 + 100 } }
impl T2 for S { fn val(&self) -> i32 { self.0 + 200 } }
fn main() { let s = S(5); println!("{} {} {} {}", s.val(), T1::val(&s), <S as T2>::val(&s), S::val(&s)); }
