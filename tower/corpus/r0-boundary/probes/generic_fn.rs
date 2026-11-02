fn maxg<T:PartialOrd>(a:T,b:T)->T{ if a>b {a} else {b} }
fn main(){ println!("{} {}", maxg(3,7), maxg(9,2)); }
