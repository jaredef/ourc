#[derive(Clone)] enum T { A(i64), B }
struct S { k: T }
fn main() { let t=T::A(3); let a=t.clone(); let b=t.clone(); let s=S{k:T::A(7)};
 println!("{} {} {}", match a { T::A(v)=>v, T::B=>0 }, match b { T::A(w)=>w, T::B=>0 }, match s.k.clone() { T::A(z)=>z, T::B=>0 }); }
