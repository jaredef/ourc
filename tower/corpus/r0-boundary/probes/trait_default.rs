trait Greet{ fn base(&self)->i32; fn full(&self)->i32{ self.base()+100 } }
struct S;
impl Greet for S{ fn base(&self)->i32{ 5 } }
fn main(){ let s=S; println!("{}", s.full()); }
