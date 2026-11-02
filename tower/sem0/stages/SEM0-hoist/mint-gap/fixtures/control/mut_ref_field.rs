struct S{v:i32}
fn bump(s:&mut S){ s.v += 10; }
fn main(){ let mut s=S{v:5}; bump(&mut s); println!("{}",s.v); }
