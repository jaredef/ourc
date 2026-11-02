mod a { pub enum E { Int(i64,u32,u32), MethodCall(Box<E>, String, Vec<E>, u32, u32) }
  pub fn mk()->E{ E::Int(138,11,12) } }
use a::*;
fn probe(src:&E)->i64{ let mut r=0; match src { E::MethodCall(_x,m,a2,_l,_h)=>{ if m.as_str()=="iter_mut" && a2.len()==0 { r=1; } }, _=>{ r=9; } } r }
fn main(){ let s=a::mk(); println!("{}", probe(&s)); }
