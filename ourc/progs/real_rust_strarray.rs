fn main() { let ks=["ab","c"]; let mut n=0; let mut i=0; while i<2 { n=n+ks[i].len(); i=i+1; } println!("{} {}", n, ks[0].len()); }
