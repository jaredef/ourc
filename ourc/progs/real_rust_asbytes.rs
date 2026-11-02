fn main() { let s="abc"; let b=s.as_bytes(); let mut n=0; let mut i=0; while i<b.len(){ if b[i]==98 { n=n+1; } i=i+1; } println!("{} {} {} {}", b.len(), b[0], b[2], n); }
