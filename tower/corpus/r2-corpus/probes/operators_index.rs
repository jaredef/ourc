use std::ops::Index;
struct A{d:[i64;3]}
impl Index<usize> for A{type Output=i64;fn index(&self,i:usize)->&i64{&self.d[i]}}
fn main(){let a=A{d:[7,8,9]};println!("{}",a[2]);}
