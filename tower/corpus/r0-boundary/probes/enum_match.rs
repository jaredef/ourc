enum E{ A(i32), B(i32,i32), C }
fn v(e:E)->i32{ match e { E::A(x)=>x, E::B(a,b)=>a+b, E::C=>0 } }
fn main(){ println!("{}", v(E::A(5))+v(E::B(3,4))+v(E::C)); }
