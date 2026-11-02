struct S{v:i32}
impl S { fn bump(&mut self){ self.v+=10; } }
fn main(){ let mut s=S{v:5}; s.bump(); println!("{}",s.v); }
