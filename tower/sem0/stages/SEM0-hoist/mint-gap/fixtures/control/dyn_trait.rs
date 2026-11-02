trait T { fn f(&self)->i32; }
struct A; impl T for A { fn f(&self)->i32{7} }
fn main(){ let b:Box<dyn T>=Box::new(A); println!("{}",b.f()); }
