fn f(n:i64)->bool{ n>0 }
fn g(n:i64)->bool{ n<0 }
fn main() { let b=f(1); println!("{} {} {:?} {}", f(1), g(1), f(0), b); }
