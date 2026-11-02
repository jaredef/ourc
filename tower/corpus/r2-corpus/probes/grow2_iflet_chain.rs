fn f(n:i64)->Option<i64>{ if n>0 {Some(n*2)} else {None} }
fn main(){ let mut s=0i64; for n in -2..3i64 { if let Some(v)=f(n) { s+=v; } } println!("{}", s); }
