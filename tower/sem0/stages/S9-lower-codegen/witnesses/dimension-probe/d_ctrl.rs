fn main() { let mut s=0; for i in 0..5 { if i%2==0 { s+=i; } else { s-=1; } } let mut k=3; while k>0 { s+=k; k-=1; } println!("{}", s); }
