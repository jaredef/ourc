fn cls(x:i64)->i64{ match x { n if n<0 => -1, 0 => 0, n if n%2==0 => 2, _ => 1 } }
fn main(){ let mut s=0i64; for i in -2..5i64 { s=s*10+ (cls(i)+2); } println!("{}", s); }
