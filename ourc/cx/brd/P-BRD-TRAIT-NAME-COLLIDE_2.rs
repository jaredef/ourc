struct S(i32);
impl S { fn val(&self) -> String { format!("inh {}", self.0) } }
trait T1 { fn val(&self) -> String; }
trait T2 { fn val(&self) -> String; }
impl T1 for S { fn val(&self) -> String { format!("T1 {}", self.0) } }
impl T2 for S { fn val(&self) -> String { format!("T2 {}", self.0) } }
fn main() { let s = S(5); println!("{} {} {} {}", s.val(), T1::val(&s), <S as T2>::val(&s), S::val(&s)); }
