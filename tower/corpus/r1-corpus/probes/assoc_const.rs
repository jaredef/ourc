trait Named{ const ID:i32; fn who(&self)->i32; }
struct A; impl Named for A{ const ID:i32=7; fn who(&self)->i32{ 100 } }
fn main(){ let a=A; println!("{}",a.who()+A::ID); }
