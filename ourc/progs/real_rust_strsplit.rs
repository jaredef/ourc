fn main(){ let s="1,2,3,4"; let v: Vec<&str> = s.split(',').collect(); let mut n=0; let mut i=0; while i<v.len() { n=n+v[i].len(); i=i+1; } println!("{} {}", v.len(), n); }
