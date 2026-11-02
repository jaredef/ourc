fn main(){ let mut s=0i64; for n in 0..10i64 { match n { 1|3|5|7|9 => s+=n, _=>{} } } println!("{}", s); }
