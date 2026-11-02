fn main(){ let mut i=0i64; let mut j=20i64; let mut steps=0; while i<j && steps<100 { i+=3; j-=1; steps+=1; } println!("{} {} {}", i, j, steps); }
